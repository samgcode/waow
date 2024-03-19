//! Represents a circle with a solid color centered at some position on the screen
//!
//! # Examples
//! ```
//! use waow::*;
//!
//! struct App {}
//! impl Run for App {
//!   fn start(&mut self, _canvas: &mut Canvas) {}
//!   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
//!     let circle = shapes::Circle::new(
//!       50, 50, 30,
//!       Color::from_rgba(1.0, 0.0, 0.0, 1.0
//!     ));
//!
//!     canvas.draw_shape(&circle);
//!   }
//! }
//! ```

use super::Drawable;
use crate::Color;

#[derive(Clone)]
pub struct Circle {
  x: i16,
  y: i16,
  #[allow(dead_code)]
  radius: i16,
  radius_squared: i32,
  color: Color,
}

impl Circle {
  /// creates a new circle
  ///
  /// Creates a new circle with the given `x` and `y` as its center,
  /// with a radius of `radius`, and a fill color of `color`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let circle = shapes::Circle::new(
  ///   50, 50, 30,
  ///   Color::from_rgba(1.0, 0.0, 0.0, 1.0
  /// ));
  /// ````
  pub fn new(x: i16, y: i16, radius: i16, color: Color) -> Self {
    return Self {
      x,
      y,
      radius,
      radius_squared: (radius * radius) as i32,
      color,
    };
  }
}

impl Drawable for Circle {
  fn get_color(&self, x: i16, y: i16) -> Option<Color> {
    let dx = (x - self.x) as i32;
    let dy = (y - self.y) as i32;

    return if (dx * dx + dy * dy) <= self.radius_squared {
      Some(self.color)
    } else {
      None
    };
  }
}
