mod math;
mod color;
mod ray;
mod camera;
mod scene;

use minifb::{Key, Window, WindowOptions};
use crate::math::Vec3;
use crate::color::Color;
use crate::math::Point3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::scene::{ Scene, AmbientLight, object::{ Object, Sphere, Cylinder } };

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 1280;

fn ray_color(ray: &Ray, scene: &Scene) -> Color {
  if let Some(_) = scene.hit(ray) {
    return Color::rgb(1.0, 0.0, 0.0); 
  }
  let unit_direction = ray.dir.unit();
  let a = 0.5 * (unit_direction.y + 1.0);
  let lerp = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(1.0, 0.5, 0.7);
  Color::Rgb(lerp)
}

fn main() {
  let mut height = (WIDTH as f64 / ASPECT_RATIO) as usize;
  height = if height < 1 { 1 } else { height };
  let actual_ratio = WIDTH as f64 / height as f64;
  
  let mut scene = Scene::new(
    Camera::new(
      Point3::new(0.0, 0.0, 0.0), 
      45.0, 
      actual_ratio,
    ),
    AmbientLight { 
      ratio: 0.2,
      color: Color::rgb(0.8, 0.8, 0.8),
    },
  );

  let camera = scene.camera.clone();
  let viewport = &camera.viewport;

  // pixel delta vectors
  let pdu = viewport.u / (WIDTH as f64);
  let pdv = viewport.v / (height as f64);
  
  // starting pixel (0,0)
  let p00 = viewport.origin + pdu / 2.0 + pdv / 2.0;
  
  // window setup
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
  
  // scene setup
  let test = Object::Sphere(Sphere { 
    center: Point3::new(-0.5, 0.0, -1.0),
    radius: 0.25 
  });
  
  let test2 = Object::Cylinder(Cylinder { 
    center: Point3::new(2.0, 1.5, -10.0),
    radius: 0.5,
    height: 3.0,
    orientation: Vec3::new(0.0, -1.0, 0.0), // normalize this 
  });

  scene.add_object(test);
  scene.add_object(test2);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    for (i,p) in buffer.iter_mut().enumerate() {
      let pixel = p00 + ((i % WIDTH) as f64) * pdu + ((i / WIDTH) as f64) * pdv;
      let ray = Ray::new(camera.position, (pixel - camera.position).unit());

      *p = ray_color(&ray, &scene).to_rgb_bytes();
    }
    window.update_with_buffer(&buffer, WIDTH, height).unwrap();
  }
}

