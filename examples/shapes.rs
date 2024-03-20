use waow::*;

fn main() {
  let app = App::create();
  create(
    app,
    CanvasConfiguration {
      width: 400,
      height: 400,
      background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
      window_name: String::from("waow!"),
    },
  );
}

struct App {
  rectangles: Vec<shapes::Rectangle>,
  circles: Vec<shapes::Circle>,
}

impl App {
  pub fn create() -> Self {
    return Self {
      rectangles: Vec::<shapes::Rectangle>::new(),
      circles: Vec::<shapes::Circle>::new(),
    };
  }
}

impl Run for App {
  fn start(&mut self, _canvas: &mut Canvas) {
    // basic rectangle
    self
      .rectangles
      .push(shapes::Rectangle::new(10, 10, 40, 20).with_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0)));

    // rectangle with border
    self.rectangles.push(
      shapes::Rectangle::new(60, 10, 40, 50)
        .with_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
        .with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 4),
    );

    // border but no fill overlapping another rect
    self.rectangles.push(
      shapes::Rectangle::new(120, 20, 40, 20).with_fill(Color::from_rgba(0.0, 1.0, 0.0, 1.0)),
    );

    self.rectangles.push(
      shapes::Rectangle::new(130, 10, 20, 40).with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 5),
    );

    // border with fill overlapping another rect
    self.rectangles.push(
      shapes::Rectangle::new(170, 20, 40, 20).with_fill(Color::from_rgba(0.0, 1.0, 0.0, 1.0)),
    );

    self.rectangles.push(
      shapes::Rectangle::new(180, 10, 20, 40)
        .with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 4)
        .with_fill(Color::from_rgba(0.0, 1.0, 1.0, 1.0)),
    );

    // basic circle
    self
      .circles
      .push(shapes::Circle::new(30, 100, 20).with_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0)));

    // circle with border
    self.circles.push(
      shapes::Circle::new(80, 100, 20)
        .with_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
        .with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 4),
    );

    // border but no fill overlapping another circle
    self
      .circles
      .push(shapes::Circle::new(150, 100, 20).with_fill(Color::from_rgba(0.0, 1.0, 0.0, 1.0)));

    self
      .circles
      .push(shapes::Circle::new(130, 100, 20).with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 5));

    // border with fill overlapping another circle
    self
      .circles
      .push(shapes::Circle::new(220, 100, 20).with_fill(Color::from_rgba(0.0, 1.0, 0.0, 1.0)));

    self.circles.push(
      shapes::Circle::new(200, 100, 20)
        .with_fill(Color::from_rgba(0.0, 1.0, 1.0, 1.0))
        .with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 5),
    );
  }

  fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
    for circle in self.circles.iter() {
      canvas.draw_shape(circle);
    }
    for rectangle in self.rectangles.iter() {
      canvas.draw_shape(rectangle);
    }
  }
}
