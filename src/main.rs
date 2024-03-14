use waow::*;

fn main() {
  let app = App {};
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

struct App {}
impl Run for App {
  fn start(&mut self, _canvas: &mut Canvas) {}
  fn draw(&mut self, canvas: &mut Canvas, input: &Input) {
    if let Some(mouse_pos) = input.mouse() {
      let mut color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
      if input.key_held(KeyCode::D) {
        color = Color::from_rgba(0.0, 1.0, 0.0, 1.0);
      }
      canvas.draw_square(mouse_pos.0 as i16, mouse_pos.1 as i16, 20, color);
    }
  }
}
