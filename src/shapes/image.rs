//! Represents a rectangle of pixels with arbitrary colors
//!
//! # Examples
//! ```
//! use waow::*;
//!
//! struct App {
//!   image: shapes::Image
//! }
//!
//! impl App {
//!   pub fn create() -> Self {
//!     return Self {
//!       image: shapes::Image::new(0, 0, 256, 256)
//!     };
//!   }
//! }
//!
//! impl Run for App {
//!   fn start(&mut self, canvas: &mut Canvas) {}
//!
//!   fn draw(&mut self, canvas: &mut Canvas, input: &Input) {
//!     for x in 0..255 {
//!      for y in 0..255 {
//!         self.image.set_pixel(x, y, Color::from_rgba_int(x as u8, y as u8, 128, 255));
//!      }
//!    }
//!    canvas.draw_shape(&self.image);
//!   }
//! }
//! ```

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
  /// creates a new image
  ///
  /// Creates a new image withe the given `x` and `y` as its top right corner,
  /// with a size of `width` x `height` and fills it with black
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let img = shapes::Image::new(30, 30, 100, 100);
  /// ```
  pub fn new(x: i16, y: i16, width: i16, height: i16) -> Self {
    let mut pixels = Vec::<Vec<Color>>::new();

    for _ in 0..width {
      let mut col = Vec::<Color>::new();
      for _ in 0..height {
        col.push(Color::from_rgba(0.0, 0.0, 0.0, 1.0));
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

  /// Sets the color of a pixel in an image
  ///
  /// Sets the `color` of (`x`, `y`) in relative coordinates
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let mut img = shapes::Image::new(0, 0, 255, 255);
  /// for x in 0..255 {
  ///   for y in 0..255 {
  ///     img.set_pixel(x, y, Color::from_rgba_int(x as u8, y as u8, 128, 255));
  ///   }
  /// }
  /// ````
  pub fn set_pixel(&mut self, x: i16, y: i16, color: Color) {
    //TODO: fix bounds check
    if x < self.x || x >= self.x + self.width || y < self.y || y >= self.y + self.height {
      panic!("Pixel position outside of image bounds ({}, {})", x, y);
    }
    self.pixels[x as usize][y as usize] = color;
  }

  //TODO: add get_pixel fn
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
