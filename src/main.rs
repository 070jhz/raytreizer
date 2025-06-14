mod math;
mod color;

use minifb::{Key, Window, WindowOptions};
use crate::math::Vec3;
use crate::color::Color;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
  let mut window = Window::new(
    "Test", 
    WIDTH, 
    HEIGHT, 
    WindowOptions {
      resize: true,
      ..WindowOptions::default()
  })
  .unwrap_or_else(|e| {
      panic!("{}", e);
  });

  window.set_target_fps(60);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    for (i,p) in buffer.iter_mut().enumerate() {
      let pixel = Color::Rgb(Vec3::new(
        (i % WIDTH) as f64 / (WIDTH - 1) as f64,
        (i / WIDTH) as f64 / (HEIGHT - 1) as f64,
        0.0,
      ));
      
      *p = pixel.to_rgb_bytes();
    }
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
  }
}

