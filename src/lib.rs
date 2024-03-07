use error_iter::ErrorIter;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
  dpi::LogicalSize,
  event::Event,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

mod canvas;
mod color;
pub mod shapes;

pub use canvas::Canvas;
pub use color::Color;
pub use winit::event::VirtualKeyCode as KeyCode;
pub use winit_input_helper::WinitInputHelper as Input;

pub struct CanvasConfiguration {
  pub width: u32,
  pub height: u32,
  pub background_color: Color,
  pub window_name: String,
}

pub trait Run {
  fn start(&mut self, canvas: &mut Canvas);
  fn draw(&mut self, canvas: &mut Canvas, input: &Input);
}

pub fn create(mut app: impl Run + 'static, config: CanvasConfiguration) {
  env_logger::init();
  let event_loop = EventLoop::new();
  let mut input = Input::new();

  let mut canvas = Canvas::new(&config);

  let (width, height) = (config.width, config.height);
  let window = {
    let size = LogicalSize::new(width as f64, height as f64);
    WindowBuilder::new()
      .with_title(config.window_name)
      .with_inner_size(size)
      .with_resizable(false)
      .with_min_inner_size(size)
      .build(&event_loop)
      .unwrap()
  };

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    Pixels::new(width, height, surface_texture).unwrap()
  };

  app.start(&mut canvas);

  event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
      canvas.draw_to_buffer(pixels.frame_mut());

      if let Err(err) = pixels.render() {
        log_error("pixels.render", err);
        *control_flow = ControlFlow::Exit;
        return;
      }
    }

    if input.update(&event) {
      if input.close_requested() {
        *control_flow = ControlFlow::Exit;
        return;
      }

      app.draw(&mut canvas, &input);
      window.request_redraw();
    }
  });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
  error!("{method_name}() failed: {err}");
  for source in err.sources().skip(1) {
    error!("  Caused by: {source}");
  }
}
