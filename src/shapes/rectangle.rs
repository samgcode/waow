use super::Drawable;
use crate::Color;

#[derive(Clone)]
pub struct Rectangle {
  x: i16,
  y: i16,
  width: i16,
  height: i16,
  color: Color,
}

impl Rectangle {
  pub fn new(x: i16, y: i16, width: i16, height: i16, color: Color) -> Self {
    return Self {
      x,
      y,
      width,
      height,
      color,
    };
  }
}

impl Drawable for Rectangle {
  fn get_color(&self, x: i16, y: i16) -> Option<Color> {
    let inside_the_box =
      x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height;
    return if inside_the_box {
      Some(self.color)
    } else {
      None
    };
  }
}
