mod square;

pub use square::Square;

pub trait Drawable {
  fn get_color(&self, x: i16, y: i16) -> Option<[u8; 4]>;
}
