use super::Drawable;

pub struct Square {
  x: i16,
  y: i16,
  size: i16,
  color: [u8; 4],
}

impl Square {
  pub fn new(x: i16, y: i16, size: i16, color: [u8; 4]) -> Self {
    return Self { x, y, size, color };
  }
}

impl Drawable for Square {
  fn get_color(&self, x: i16, y: i16) -> Option<[u8; 4]> {
    let inside_the_box =
      x >= self.x && x < self.x + self.size && y >= self.y && y < self.y + self.size;
    return if inside_the_box {
      Some(self.color)
    } else {
      None
    };
  }
}
