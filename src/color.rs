//! Represents a pixel color
//!
//! There are multiple methods for creating colors, with
//! different input types

#[derive(Clone, Copy)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
  a: f64,
}

/// Represents the color of a pixel
impl Color {
  /// Creates a color from rgba values from `0.0` to `1.0` as `f64`
  ///
  /// # Panics
  /// panics if any of the input values are not within the range `0.0` to `1.0`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let col = Color::from_rgba(0.5, 0.75, 0.25, 1.0);
  /// ```
  pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
    assert!(
      r <= 1.0 && r >= 0.0,
      "color values must be within the range 0.0 - 1.0"
    );
    assert!(
      g <= 1.0 && g >= 0.0,
      "color values must be within the range 0.0 - 1.0"
    );
    assert!(
      b <= 1.0 && b >= 0.0,
      "color values must be within the range 0.0 - 1.0"
    );
    assert!(
      a <= 1.0 && a >= 0.0,
      "color values must be within the range 0.0 - 1.0"
    );

    return Self { r, g, b, a };
  }

  /// Creates a color from rgba values from `0.0` to `1.0` as `f32`
  ///
  /// # Panics
  /// panics if any of the input values are not within the range `0.0` to `1.0`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let col = Color::from_rgba32(0.5, 0.75, 0.25, 1.0);
  /// ```
  pub fn from_rgba32(r: f32, g: f32, b: f32, a: f32) -> Self {
    return Self::from_rgba(r as f64, a as f64, b as f64, g as f64);
  }

  /// Creates a color from rgba values from `0` to `255` as `u8`
  ///
  /// # Panics
  /// panics if any of the input values are not within the range `0` to `255`
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// let col = Color::from_rgba_int(128, 200, 64, 255);
  /// ```
  pub fn from_rgba_int(r: u8, g: u8, b: u8, a: u8) -> Self {
    return Self::from_rgba(
      r as f64 / 255.0,
      g as f64 / 255.0,
      b as f64 / 255.0,
      a as f64 / 255.0,
    );
  }

  /// returns the color as an array of u8
  pub fn as_bytes(&self) -> [u8; 4] {
    return [
      (self.r * 255.0) as u8,
      (self.g * 255.0) as u8,
      (self.b * 255.0) as u8,
      (self.a * 255.0) as u8,
    ];
  }
}
