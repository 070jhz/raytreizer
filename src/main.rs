use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn rgb_from_u8(r: u8, g: u8, b: u8) -> u32 {
  let (r, g, b) = (r as u32, g as u32, b as u32);
  (r << 16) | (g << 8) | b
}

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
    for (i, p) in buffer.iter_mut().enumerate() {
      *p = rgb_from_u8(
        ((i % WIDTH) as f32 / (WIDTH-1) as f32 * 255.0) as u8,
        ((i / WIDTH) as f32 / (HEIGHT-1) as f32 * 255.0) as u8,
        0,
      );
    }
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
  }
}

