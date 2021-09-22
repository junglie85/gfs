use log::error;
use pixels::Error;
use winit::dpi::LogicalSize;
use winit::event::{VirtualKeyCode, Event};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::canvas::Canvas;

pub trait App {
    fn update(&self, canvas: &mut Canvas);
}

pub fn run<T: 'static + App>(width: u32, height: u32, title: &str, app: T) -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut canvas = Canvas::new(&window, width, height)?;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if let Event::RedrawRequested(_) = event {
            if canvas.update()
                .map_err(|e| error!("canvas.update() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        app.update(&mut canvas);

        window.request_redraw();
    });
}
