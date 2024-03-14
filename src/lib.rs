//! A simple api for creating and drawing to a window
//!
//! waow provides functionality for creating a window,
//! and preforming various operations about it such as
//! drawing shapes, and reading user input
//!
//! For a quick start, see the [examples](https://github.com/samgcode/waow/tree/master/examples)

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

/// Canvas that can be drawn to and gets displayed on the screen
pub use canvas::Canvas;
/// represents an RGBA color
pub use color::Color;
/// Represents an input key
pub use winit::event::VirtualKeyCode as KeyCode;
pub use winit_input_helper::WinitInputHelper as Input;

/// Holds the configuration of the canvas.
///
/// Passed into the [`create()`] method to set the various
/// attributes of the canvas
///
/// # Examples
/// ```
/// use waow::*;
///
/// let app = App {};
/// create(
///   app,
///   CanvasConfiguration {
///     width: 400,
///     height: 400,
///     background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
///     window_name: String::from("waow!"),
///   },
/// );
///
/// struct App {}
/// impl Run for App {
///   fn start(&mut self, _canvas: &mut Canvas) {}
///   fn draw(&mut self, _canvas: &mut Canvas, _input: &Input) {}
/// }
/// ```
pub struct CanvasConfiguration {
  pub width: u32,
  pub height: u32,
  pub background_color: Color,
  pub window_name: String,
}

/// Defines the behavior for an app that effects the canvas
///
/// The `start()` method is called before the first frame
/// The `draw()` method is called every frame
///
/// # Examples
/// ```
/// use waow::*;
///
/// struct App {}
/// impl Run for App {
///   fn start(&mut self, _canvas: &mut Canvas) {}
///   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
///     canvas.draw_square(50, 50, 20, Color::from_rgba(1.0, 0.0, 0.0, 1.0))
///   }
/// }
/// ```
pub trait Run {
  fn start(&mut self, canvas: &mut Canvas);
  fn draw(&mut self, canvas: &mut Canvas, input: &Input);
}

/// Creates a canvas to draw on
///
/// Creates a new window with the given [`CanvasConfiguration`]
/// and runs the app with the canvas
/// # Examples
/// ```
/// use waow::*;
///
/// fn main() {
///   let app = App {};
///   create(
///     app,
///     CanvasConfiguration {
///       width: 400,
///       height: 400,
///       background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
///       window_name: String::from("waow!"),
///     },
///   );
/// }
///
/// struct App {}
/// impl Run for App {
///   fn start(&mut self, _canvas: &mut Canvas) {}
///   fn draw(&mut self, canvas: &mut Canvas, input: &Input) {
///     if let Some(mouse_pos) = input.mouse() {
///       let mut color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
///       if input.key_held(KeyCode::D) {
///         color = Color::from_rgba(0.0, 1.0, 0.0, 1.0);
///       }
///       canvas.draw_square(mouse_pos.0 as i16, mouse_pos.1 as i16, 20, color);
///     }
///   }
/// }
/// ```
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
