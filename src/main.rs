mod math;
mod color;
mod ray;

use minifb::{Key, Window, WindowOptions};
use crate::math::Vec3;
use crate::color::Color;
use crate::math::Point3;
use crate::ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 1280;
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(ray: &Ray) -> Color {
  let unit_direction = ray.dir.unit();
  let a = 0.5 * (unit_direction.y + 1.0);
  let lerp = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(1.0, 0.5, 0.7);
  Color::Rgb(lerp)
}

fn main() {
  let mut height = (WIDTH as f64 / ASPECT_RATIO) as usize;
  height = if height < 1 { 1 } else { height };
  
  // CAMERA

  // start at upper-left pixel (0, 0)
  // scan left-to-right along vector from left edge to right edge (viewport_u)
  // row-by-row, along vector from upper edge to lower edge (viewport_v)
  // since we navigate the pixel grid top-to-bottom, Y increases going *down* the image, hence the
  // minus sign.
  
  // viewport needs to be calculated from real aspect ratio
  let actual_ratio = WIDTH as f64 / height as f64;
  let viewport_width: f64 = VIEWPORT_HEIGHT * actual_ratio;

  let camera_center = Point3::new(0.0, 0.0, 0.0);
  let vp_u = Vec3::new(viewport_width, 0.0, 0.0);
  let vp_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
  
  // pixel delta vectors
  let pdu = vp_u / (WIDTH as f64);
  let pdv = vp_v / (height as f64);

  // top left (viewpoint corner + pixel)
  let vp_top_left = camera_center
                    - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - vp_u / 2.0 - vp_v / 2.0; 
  let p00 = vp_top_left + pdu / 2.0 + pdv / 2.0;
  
  let mut buffer: Vec<u32> = vec![0; WIDTH * height];
  let mut window = Window::new(
    "Test",
    WIDTH, 
    height, 
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
      let pixel = p00 + ((i % WIDTH) as f64) * pdu + ((i / WIDTH) as f64) * pdv;
      let ray = Ray::new(camera_center, pixel - camera_center);

      *p = ray_color(&ray).to_rgb_bytes();
    }
    window.update_with_buffer(&buffer, WIDTH, height).unwrap();
  }
}

