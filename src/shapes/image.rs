use super::Drawable;
use crate::Color;

#[derive(Clone)]
pub struct Image {
  x: i16,
  y: i16,
  width: i16,
  height: i16,
  pixels: Vec<Vec<Color>>,
}

impl Image {
  pub fn new(x: i16, y: i16, width: i16, height: i16) -> Self {
    let mut pixels = Vec::<Vec<Color>>::new();

    for _ in 0..width {
      let mut col = Vec::<Color>::new();
      for _ in 0..height {
        col.push(Color::from_rgba(0.0, 0.0, 0.0, 1.0, false));
      }

      pixels.push(col);
    }

    return Self {
      x,
      y,
      width,
      height,
      pixels,
    };
  }

  pub fn set_pixel(&mut self, x: i16, y: i16, color: Color) {
    if x < self.x || x >= self.x + self.width || y < self.y || y >= self.y + self.height {
      panic!("Pixel position outside of image bounds");
    }
    self.pixels[x as usize][y as usize] = color;
  }
}

impl Drawable for Image {
  fn get_color(&self, x: i16, y: i16) -> Option<Color> {
    if x < self.x || x >= self.x + self.width || y < self.y || y >= self.y + self.height {
      return None;
    }
    Some(
      *self
        .pixels
        .get(x as usize)
        .unwrap()
        .get(y as usize)
        .unwrap(),
    )
  }
}
