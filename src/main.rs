mod math;
mod color;
mod ray;
mod camera;
mod scene;
mod framebuffer;
mod utils;

use std::sync::Arc;

use minifb::{Key, Window, WindowOptions};
use crate::math::Vec3;
use crate::color::Color;
use crate::math::Point3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::scene::{ Scene, AmbientLight, object::{ Object, Sphere, Cylinder, Plane } };

use self::scene::material::{BasicMetal, Solid};

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
    center: Point3::new(-1.1, 0.0, -2.5),
    radius: 0.5,
    material: Arc::new( 
      BasicMetal {
        albedo: Color::rgb(0.8, 0.8, 0.8),
        fuzz: 0.05,
      }
    ),
  };

  let sp2 = Sphere {
    center: Point3::new(1.1, 0.0, -2.5),
    radius: 0.5,
    material: Arc::new(
      BasicMetal {
        albedo: Color::rgb(0.8, 0.6, 0.2),
        fuzz: 0.05,
      }
    ), 
  };

  let sp3 = Sphere {
    center: Point3::new(0.0, -100.5, -2.5),
    radius: 100.0,
    material: Arc::new(
      BasicMetal {
        albedo: Color::rgb(0.8, 0.8, 0.0),
        fuzz: 0.05,
      }
    ),
  };
 
  let sp4 = Sphere {
    center: Point3::new(0.0, 0.0, -2.5),
    radius: 0.5,
    material: Arc::new(
      BasicMetal {
        albedo: Color::rgb(0.1, 0.2, 0.5),
        fuzz: 0.5,
      }
    ),
  };

  scene.add_object(Object::Sphere(sp));
  scene.add_object(Object::Sphere(sp2));
  scene.add_object(Object::Sphere(sp3));
  scene.add_object(Object::Sphere(sp4));
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

