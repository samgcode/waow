use crate::Color;

mod image;
mod rectangle;

pub use image::Image;
pub use rectangle::Rectangle;

pub trait Drawable {
  fn get_color(&self, x: i16, y: i16) -> Option<Color>;
}
