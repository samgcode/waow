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
  fill: Option<Color>,
  border: Option<Color>,
  border_width: i16,
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
  pub fn new(x: i16, y: i16, width: i16, height: i16) -> Self {
    return Self {
      x,
      y,
      width,
      height,
      fill: None,
      border: None,
      border_width: 0,
    };
  }

  pub fn with_fill(mut self, fill: Color) -> Self {
    self.fill = Some(fill);
    return self;
  }

  pub fn with_border(mut self, border: Color, width: i16) -> Self {
    self.border = Some(border);
    self.border_width = width;
    return self;
  }
}

impl Drawable for Rectangle {
  fn get_color(&self, x: i16, y: i16) -> Option<Color> {
    if x < self.x || y < self.y || x >= self.x + self.width || y >= self.y + self.height {
      return None;
    }

    if let Some(_) = self.border {
      if x < self.x + self.border_width
        || y < self.y + self.border_width
        || x >= self.x + self.width - self.border_width
        || y >= self.y + self.height - self.border_width
      {
        return self.border;
      }
    }

    if let Some(_) = self.fill {
      return self.fill;
    }

    return None;
  }
}
