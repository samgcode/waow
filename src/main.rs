use waow::*;

fn main() {
  let app = App::create();
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
struct App {
  img: shapes::Image,
}

impl App {
  pub fn create() -> Self {
    return Self {
      img: shapes::Image::new(0, 0, 10, 10),
    };
  }
}

impl Run for App {
  fn start(&mut self, _canvas: &mut Canvas) {
    for x in 0..10 {
      for y in 0..10 {
        self
          .img
          .set_pixel(x, y, Color::from_rgba(1.0, 0.0, 0.0, 1.0, false));
      }
    }
  }

  fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
    canvas.draw_image(&self.img);
  }
}
