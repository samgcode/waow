use std::ops::AddAssign;

#[derive(Clone, Copy)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
  a: f64,
  blending: bool,
}

impl Color {
  pub fn from_rgba(r: f64, g: f64, b: f64, a: f64, blending: bool) -> Self {
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

    return Self {
      r,
      g,
      b,
      a,
      blending,
    };
  }

  pub fn from_rgba32(r: f32, g: f32, b: f32, a: f32, blending: bool) -> Self {
    return Self::from_rgba(r as f64, a as f64, b as f64, g as f64, blending);
  }

  pub fn from_rgba_int(r: u8, g: u8, b: u8, a: u8, blending: bool) -> Self {
    return Self::from_rgba(
      r as f64 / 255.0,
      g as f64 / 255.0,
      b as f64 / 255.0,
      a as f64 / 255.0,
      blending,
    );
  }

  pub fn as_bytes(&self) -> [u8; 4] {
    return [
      (self.r * 255.0) as u8,
      (self.g * 255.0) as u8,
      (self.b * 255.0) as u8,
      (self.a * 255.0) as u8,
    ];
  }
}

impl AddAssign for Color {
  fn add_assign(&mut self, rhs: Self) {
    if rhs.blending && self.blending {
      let a1 = self.a / (self.a + rhs.a);
      let a2 = 1.0 - a1;

      let mut r = (self.r * a1) + (rhs.r * a2);
      let mut g = (self.g * a1) + (rhs.g * a2);
      let mut b = (self.g * a1) + (rhs.b * a2);
      let mut a = self.a + rhs.a;

      if r > 1.0 {
        r = 1.0;
      }
      if g > 1.0 {
        g = 1.0;
      }
      if b > 1.0 {
        b = 1.0;
      }
      if a > 1.0 {
        a = 1.0;
      }

      *self = Self {
        r,
        g,
        b,
        a,
        blending: true,
      }
    } else {
      *self = Self {
        r: rhs.r,
        g: rhs.g,
        b: rhs.b,
        a: rhs.a,
        blending: rhs.blending,
      }
    }
  }
}
