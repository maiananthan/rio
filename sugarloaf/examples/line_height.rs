use rio_window::keyboard::{Key, NamedKey};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use rio_window::application::ApplicationHandler;
use rio_window::event_loop::{ActiveEventLoop, ControlFlow, DeviceEvents};
use rio_window::window::{Window, WindowId};
use rio_window::{
    dpi::LogicalSize, event::{WindowEvent, ElementState}, event_loop::EventLoop, window::WindowAttributes,
};
use std::error::Error;
use sugarloaf::{
    layout::RootStyle, FragmentStyle, Object, RichText,
    Sugarloaf, SugarCursor, SugarloafWindow, SugarloafWindowSize,
};

fn main() {
    let width = 600.0;
    let height = 400.0;
    let window_event_loop = rio_window::event_loop::EventLoop::new().unwrap();
    let mut application = Application::new(&window_event_loop, width, height);
    let _ = application.run(window_event_loop);
}

struct Application {
    sugarloaf: Option<Sugarloaf<'static>>,
    window: Option<Window>,
    height: f32,
    width: f32,
    line_height: f32,
}

impl Application {
    fn new(event_loop: &EventLoop<()>, width: f32, height: f32) -> Self {
        event_loop.listen_device_events(DeviceEvents::Never);

        Application {
            sugarloaf: None,
            window: None,
            width,
            height,
            line_height: 2.0,
        }
    }

    fn run(&mut self, event_loop: EventLoop<()>) -> Result<(), Box<dyn Error>> {
        let result = event_loop.run_app(self);
        result.map_err(Into::into)
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        let window_attribute = WindowAttributes::default()
            .with_title("Line height example")
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_resizable(true);
        let window = active_event_loop.create_window(window_attribute).unwrap();

        let scale_factor = window.scale_factor();
        let font_size = 24.;

        let sugarloaf_layout =
            RootStyle::new(scale_factor as f32, font_size, self.line_height);

        let size = window.inner_size();
        let sugarloaf_window = SugarloafWindow {
            handle: window.window_handle().unwrap().into(),
            display: window.display_handle().unwrap().into(),
            scale: scale_factor as f32,
            size: SugarloafWindowSize {
                width: size.width as f32,
                height: size.height as f32,
            },
        };

        let mut sugarloaf = Sugarloaf::new(
            sugarloaf_window,
            sugarloaf::SugarloafRenderer::default(),
            &sugarloaf::font::FontLibrary::default(),
            sugarloaf_layout,
        )
        .expect("Sugarloaf instance should be created");

        sugarloaf.set_background_color(Some(wgpu::Color::BLUE));
        sugarloaf.create_rich_text();
        window.request_redraw();

        self.sugarloaf = Some(sugarloaf);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.sugarloaf.is_none() || self.window.is_none() {
            return;
        }

        let sugarloaf = self.sugarloaf.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ScaleFactorChanged {
                // mut inner_size_writer,
                scale_factor,
                ..
            } => {
                let scale_factor_f32 = scale_factor as f32;
                let new_inner_size = window.inner_size();
                sugarloaf.rescale(scale_factor_f32);
                sugarloaf.resize(new_inner_size.width, new_inner_size.height);
                window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                sugarloaf.resize(new_size.width, new_size.height);
                window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                is_synthetic: false,
                event: key_event,
                ..
            } => {
                if key_event.state == ElementState::Pressed {
                    match key_event.logical_key.as_ref() {
                        Key::Named(NamedKey::ArrowUp) => {
                            self.line_height += 0.1;
                            window.request_redraw();
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            if self.line_height > 1.0 {
                                self.line_height -= 0.1;
                                window.request_redraw();
                            }
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested { .. } => {
                let content = sugarloaf.content();
                content.sel(0).clear();
                content
                    .new_line()
                    .add_text(
                        &format!("current line_height: {:?}", self.line_height),
                        FragmentStyle {
                            color: [0.0, 0.0, 0.0, 1.0],
                            background_color: Some([1.0, 1.0, 1.0, 1.0]),
                            ..FragmentStyle::default()
                        },
                    )
                    .new_line()
                    .add_text(
                        "press arrow up to increase",
                        FragmentStyle {
                            color: [1.0, 1.0, 1.0, 1.0],
                            background_color: Some([0.0, 0.0, 0.0, 1.0]),
                            ..FragmentStyle::default()
                        },
                    )
                    .new_line()
                    .add_text(
                        "press arrow down to decrease",
                        FragmentStyle {
                            color: [0.0, 0.0, 0.0, 1.0],
                            background_color: Some([1.0, 1.0, 1.0, 1.0]),
                            ..FragmentStyle::default()
                        },
                    )
                    .new_line()
                    .add_text(
                        "│ \u{E0B6}Hello There!\u{e0b4}",
                        FragmentStyle {
                            color: [1.0, 1.0, 1.0, 1.0],
                            background_color: Some([1.0, 0.5, 1.0, 1.0]),
                            ..FragmentStyle::default()
                        },
                    )
                    .add_text(
                        "?",
                        FragmentStyle {
                            color: [0.5, 0.5, 1.0, 1.0],
                            background_color: Some([1.0, 1.0, 1.0, 1.0]),
                            cursor: Some(SugarCursor::Block([1.0, 1.0, 1.0, 1.0])),
                            ..FragmentStyle::default()
                        },
                    )
                    .build();

                sugarloaf.set_objects(vec![Object::RichText(RichText {
                    id: 0,
                    position: [10., 0.],
                })]);
                sugarloaf.render();
                event_loop.set_control_flow(ControlFlow::Wait);
            }
            _ => (),
        }
    }
}
