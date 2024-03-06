use waow::*;

fn main() {
  create(
    start,
    draw,
    CanvasConfiguration {
      width: 400,
      height: 400,
      background_color: [128, 0, 128, 255],
      window_name: String::from("waow!"),
    },
  );
}

fn start(_canvas: &mut Canvas) {
  println!("start");
}

fn draw(canvas: &mut Canvas) {
  canvas.draw_square(50, 50, 20, [255, 0, 0, 255]);
}
