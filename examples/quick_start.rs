// include the library
use waow::*;

fn main() {
  // instantiate the custom code
  let app = App::create();

  // set the configuration for the window
  let config = CanvasConfiguration {
    width: 400,
    height: 400,
    background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
    window_name: String::from("a colored square, waow!"),
  };

  // create a window with the given configuration
  create(app, config);
}

// define the struct to hold the state for your custom code
struct App {}

impl App {
  // define the function for starting the app
  pub fn create() -> Self {
    return Self {};
  }
}

impl Run for App {
  // start is called before the first frame
  fn start(&mut self, _canvas: &mut Canvas) {}

  // draw is called before every frame
  fn draw(&mut self, canvas: &mut Canvas, input: &Input) {
    // get the mouse position if the mouse is over the window
    if let Some(mouse_pos) = input.mouse() {
      // get the width and height of the canvas
      let (width, height) = canvas.get_size();

      // map the cursor position to between 0.0 and 1.0
      let r = mouse_pos.0 as f64 / width as f64;
      let b = mouse_pos.1 as f64 / height as f64;

      // set the color based on the cursor position
      let color = Color::from_rgba(r, 0.5, b, 1.0);

      // show a square if the user isn't pressing space
      if !input.key_held(KeyCode::Space) {
        // draw a square centered at the cursor position
        canvas.draw_square(
          (mouse_pos.0 - 10.0) as i16,
          (mouse_pos.1 - 10.0) as i16,
          20,
          color,
          None,
        );
      }
    }
  }
}
