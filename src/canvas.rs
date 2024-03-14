//! The canvas is the object that controls what is drawn to
//! the screen
//!
//! The canvas contains methods for drawing certain shapes,
//! as well as drawing any [`Drawable`]

use crate::{
  shapes::{Drawable, Image, Rectangle},
  CanvasConfiguration, Color,
};

pub struct Canvas {
  width: u32,
  height: u32,
  background_color: Color,
  objects: Vec<Box<dyn Drawable>>,
}

impl Canvas {
  pub fn new(config: &CanvasConfiguration) -> Self {
    return Self {
      width: config.width,
      height: config.height,
      background_color: config.background_color,
      objects: Vec::<Box<dyn Drawable>>::new(),
    };
  }

  /// Gets the (width, height) of the canvas in pixels
  pub fn get_size(&self) -> (u32, u32) {
    return (self.width, self.height);
  }

  pub fn draw_to_buffer(&mut self, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = (i % self.width as usize) as i16;
      let y = (i / self.height as usize) as i16;

      let mut rgba = self.background_color;
      for object in self.objects.iter() {
        if let Some(color) = object.as_ref().get_color(x, y) {
          rgba = color;
        }
      }

      pixel.copy_from_slice(&rgba.as_bytes());
    }

    self.objects = Vec::<Box<dyn Drawable>>::new();
  }
}

impl Canvas {
  /// Draws an arbitrary shape to the canvas
  ///
  /// See [`Drawable`] for examples
  pub fn draw_shape(&mut self, shape: &(impl Drawable + Clone + 'static)) {
    self.objects.push(Box::new(shape.clone()));
  }

  /// Draws a square with the specified position, size, and color
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// struct App {}
  /// impl Run for App {
  ///   fn start(&mut self, _canvas: &mut Canvas) {}
  ///   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
  ///     canvas.draw_square(10, 10, 30, Color::from_rgba(0.0, 1.0, 0.5, 1.0));
  ///   }
  /// }
  /// ```
  pub fn draw_square(&mut self, x: i16, y: i16, size: i16, color: Color) {
    self.draw_shape(&Rectangle::new(x, y, size, size, color));
  }

  /// Draws a rectangle with the specified position, width, height, and color
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// struct App {}
  /// impl Run for App {
  ///   fn start(&mut self, _canvas: &mut Canvas) {}
  ///   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
  ///     canvas.draw_rect(50, 10, 20, 50, Color::from_rgba(0.0, 0.5, 1.0, 1.0));
  ///   }
  /// }
  /// ```
  pub fn draw_rect(&mut self, x: i16, y: i16, width: i16, height: i16, color: Color) {
    self.draw_shape(&Rectangle::new(x, y, width, height, color));
  }

  /// Draws an image object to the canvas
  ///
  /// # Examples
  /// ```
  /// use waow::*;
  ///
  /// struct App {}
  /// impl Run for App {
  ///   fn start(&mut self, _canvas: &mut Canvas) {}
  ///   fn draw(&mut self, canvas: &mut Canvas, _input: &Input) {
  ///     let img = shapes::Image::new(30, 30, 100, 100);
  ///
  ///     canvas.draw_image(&img);
  ///   }
  /// }
  /// ```
  pub fn draw_image(&mut self, image: &Image) {
    self.draw_shape(image);
  }
}
