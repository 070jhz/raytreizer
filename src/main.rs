mod math;
mod color;
mod ray;
mod camera;
mod scene;
mod framebuffer;
mod utils;

use minifb::{Key, Window, WindowOptions};
use crate::math::Vec3;
use crate::color::Color;
use crate::math::Point3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::scene::{ Scene, AmbientLight, object::{ Object, Sphere, Cylinder, Plane } };

fn main() {
  let mut scene = Scene::new(
    Camera::new(
      Point3::new(0.0, 0.0, 0.0), 
      45.0, 
      16.0 / 9.0,
      1280,
    ),
    AmbientLight { 
      ratio: 0.2,
      color: Color::rgb(0.8, 0.8, 0.8),
    },
  );  

  let sp = Sphere {
    center: Point3::new(0.0, 0.0, -1.0),
    radius: 0.25,
  };

  let cyl = Cylinder {
    center: Point3::new(-0.7, 0.5, -1.0),
    radius: 0.125,
    height: 0.5,
    orientation: Vec3::new(0.2, 0.0, -1.0).unit(),
  };

  scene.add_object(Object::Sphere(sp));
  scene.add_object(Object::Cylinder(cyl));

  let camera = &scene.camera;
  
  // window setup
  let mut window = Window::new(
    "Test",
    camera.image_width,
    camera.image_height, 
    WindowOptions {
      resize: true,
      ..WindowOptions::default()
  })
  .unwrap_or_else(|e| {
      panic!("{}", e);
  });
  window.set_target_fps(60);
  
 
  while window.is_open() && !window.is_key_down(Key::Escape) {
    let buffer = scene.render_frame();
    window.update_with_buffer(&buffer.buf, camera.image_width, camera.image_height).unwrap();
  }
}

