//! provides an api for creating shapes to be drawn
//!
//! shapes provides a list of [Structs](#Structs) that can be
//! drawn to the screen. It also provides the [`Drawable`] trait
//! which allows for creating objects with custom behaviour

use crate::Color;

mod image;
mod rectangle;

/// public reexports
pub use image::Image;
pub use rectangle::Rectangle;

/// Defines shared behavior for drawable objects
///
/// the [`get_color()`] method takes in pixel coordinates on the screen
/// and returns the color of the object at that coordinate
///
/// returns `None` if the position is not within the object
///
/// # Examples
/// ```
/// use waow::*;
///
/// #[derive(Clone)]
/// struct DrawEvenX {
///   color: Color
/// }
///
/// impl shapes::Drawable for DrawEvenX {
///   fn get_color(&self, x: i16, y: i16) -> Option<Color> {
///     if x % 2 == 0 {
///       return Some(self.color);
///     } else {
///       return None;
///     }
///   }
/// }
/// ````
pub trait Drawable {
  fn get_color(&self, x: i16, y: i16) -> Option<Color>;
}
