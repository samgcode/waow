# waow

`waow` is a creative coding library that is intended to be simple and beginner friendly.

Heres what you can do with waow:

- Draw simple shapes to the screen
  - currently: rectangle, but more will be added soon
- Draw an image (essentially an array of pixels)
- Create custom shapes with the `Drawable` trait
- Get user input using the `Input` module

# Quick start

In cargo.toml:

```toml
[dependencies]
waow="0.7.0"
```

Then in main.rs:

```rs
// import waow
use waow::*;

// create the window
fn main() {
  // initialize the code for the library to run
  let app = App::create();

  // create the window and start the draw loop
  create(
    app,
    CanvasConfiguration {
      width: 400,
      height: 400,
      background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0, false),
      window_name: String::from("waow!"),
    },
  );
}

// the struct that holds your code
struct App {}

// add a method for creating the app
impl App {
  pub fn create() -> Self {
    return Self {};
  }
}

// implement the Run trait so your code can effect thr canvas
impl Run for App {
  // ran before the first frame is drawn
  fn start(&mut self, canvas: &mut Canvas) {

  }

  // ran every time a frame is drawn
  fn draw(&mut self, canvas: &mut Canvas, input: &Input) {

  }
}
```

For info on how to use all of the features, check out the [examples](!todo), or read the [docs](!todo)
