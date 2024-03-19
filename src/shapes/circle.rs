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
  fill: Option<Color>,
  border: Option<Color>,
  border_width: i16,
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
  /// let circle = shapes::Circle::new(50, 50, 30);
  /// ````
  pub fn new(x: i16, y: i16, radius: i16) -> Self {
    return Self {
      x,
      y,
      radius,
      radius_squared: (radius * radius) as i32,
      fill: None,
      border: None,
      border_width: 0,
    };
  }

  /// A builder that adds a fill to a rectangle
  ///
  /// Takes an instance of a rectangle and returns an instance with
  /// the fill set to `color`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let rectangle = shapes::Rectangle::new(10, 10, 30, 50)
  ///   .with_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0));
  /// ```
  pub fn with_fill(mut self, color: Color) -> Self {
    self.fill = Some(color);
    return self;
  }

  /// A builder that adds a border to a rectangle
  ///
  /// Takes an instance of a rectangle and returns an instance with
  /// the border color set to `color` and border width set to `width`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let rectangle = shapes::Rectangle::new(10, 10, 30, 50)
  ///   .with_border(Color::from_rgba(0.0, 0.0, 1.0, 1.0), 4);
  /// ```
  pub fn with_border(mut self, color: Color, width: i16) -> Self {
    self.border = Some(color);
    self.border_width = width;
    return self;
  }
}

impl Drawable for Circle {
  fn get_color(&self, x: i16, y: i16) -> Option<Color> {
    let dx = (x - self.x) as i32;
    let dy = (y - self.y) as i32;
    let dist_sq = dx * dx + dy * dy;

    if dist_sq > self.radius_squared {
      return None;
    }

    if let Some(_) = self.border {
      if dist_sq > ((self.radius - self.border_width) * (self.radius - self.border_width)) as i32 {
        return self.border;
      }
    }

    if let Some(_) = self.fill {
      return self.fill;
    }

    return None;
  }
}
