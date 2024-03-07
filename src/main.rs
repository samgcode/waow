use waow::*;

fn main() {
  create(
    start,
    draw,
    CanvasConfiguration {
      width: 400,
      height: 400,
      background_color: Color::from_rgba(0.0, 0.0, 0.0, 1.0, false),
      window_name: String::from("waow!"),
    },
  );
}

fn start(_canvas: &mut Canvas) {
  println!("start");
}

fn draw(canvas: &mut Canvas, _input: &Input) {
  let mut img = shapes::Image::new(0, 0, 10, 10);

  for x in 0..10 {
    for y in 0..10 {
      img.set_pixel(x, y, Color::from_rgba(1.0, 0.0, 0.0, 1.0, false));
    }
  }

  canvas.draw_shape(img);
}
