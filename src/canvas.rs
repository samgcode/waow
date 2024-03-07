use crate::{
  shapes::{Drawable, Rectangle},
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
          rgba += color;
        }
      }

      pixel.copy_from_slice(&rgba.as_bytes());
    }

    self.objects = Vec::<Box<dyn Drawable>>::new();
  }
}

impl Canvas {
  pub fn draw_shape(&mut self, shape: impl Drawable + 'static) {
    self.objects.push(Box::new(shape));
  }

  pub fn draw_square(&mut self, x: i16, y: i16, size: i16, color: Color) {
    self.draw_shape(Rectangle::new(x, y, size, size, color));
  }

  pub fn draw_rect(&mut self, x: i16, y: i16, width: i16, height: i16, color: Color) {
    self.draw_shape(Rectangle::new(x, y, width, height, color));
  }
}
