use crate::Color;
use crate::Camera;
use crate::Ray;
use crate::scene::object::{ Object, HitRecord, Hittable };

pub mod light;
pub mod object;
pub mod parser;

pub struct AmbientLight {
  pub ratio: f64,
  pub color: Color,
}

pub struct Scene {
  pub camera  : Camera,
  pub ambient : AmbientLight,
  
  pub objects : Vec<Object>,
  //pub lights: Vec<Light>,
}

impl Scene {
  pub fn new(camera: Camera, ambient: AmbientLight) -> Self {
    Scene {
      camera,
      ambient,
      objects: Vec::new(),
      //lights: Vec::new(),
    }
  }

  pub fn add_object(&mut self, object: Object) {
    self.objects.push(object);  
  }

  pub fn clear(&mut self) {
    self.objects.clear();
    //self.lights.clear();
  }

  pub fn hit(&self,ray: &Ray) -> Option<HitRecord> {
    let mut closest   : Option<HitRecord> = None;
    let mut closest_t = f64::INFINITY;
    
    for object in &self.objects {
      if let Some(hit) = object.hit(ray) {
        if hit.t < closest_t {
          closest_t = hit.t;
          closest = Some(hit);
        }
      }
    }

    closest
  }
}
