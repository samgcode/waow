//! Represents a rectangle with a solid color at some position on the screen
//!
//! # Examples
//! ```
//! use waow::*;
//!
//! struct App {}
//! impl Run for App {
//!   fn start(&mut self, _canvas: &mut Canvas) {}
//!   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
//!     let rectangle = shapes::Rectangle::new(
//!       10, 10, 30, 50,
//!       Color::from_rgba(1.0, 0.0, 0.0, 1.0
//!     ));
//!
//!     canvas.draw_shape(&rectangle);
//!   }
//! }
//! ```

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
  /// creates a new rectangle
  ///
  /// Creates a new rectangle withe the given `x` and `y` as its top right corner,
  /// with a size of `width` x `height`, and a fill color of `color`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let rectangle = shapes::Rectangle::new(
  ///   10, 10, 30, 50,
  ///   Color::from_rgba(1.0, 0.0, 0.0, 1.0
  /// ));
  /// ````
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
