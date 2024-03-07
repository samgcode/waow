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

fn draw(canvas: &mut Canvas, input: &Input) {
  if let Some(mouse_position) = input.mouse() {
    if input.key_held(KeyCode::C) {
      println!("mouse pos: {:?}", mouse_position);
    }
  }
}
