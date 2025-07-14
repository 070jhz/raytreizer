use crate::framebuffer::FrameBuffer;
use crate::utils::random_double_in;
use crate::Color;
use crate::Camera;
use crate::Ray;
use crate::scene::object::{ Object, HitRecord, Hittable };
use crate::math::Vec3;

pub mod light;
pub mod object;
//pub mod parser;
pub mod material;

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

  pub fn render_frame(&self) -> FrameBuffer {
    let mut buffer = FrameBuffer::new(self.camera.image_width, self.camera.image_height);
    for j in 0..self.camera.image_height {
      for i in 0..self.camera.image_width {
        // anti aliasing
        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

        for _sample in 0..self.camera.sampling_rate {
          let pixel = self.camera.viewport.p00
                    + (i as f64 + random_double_in(-0.5, 0.5)) * self.camera.viewport.pdu
                    + (j as f64 + random_double_in(-0.5, 0.5)) * self.camera.viewport.pdv;
          let ray = Ray::new(self.camera.position, (pixel - self.camera.position).unit());
          pixel_color += self.ray_color(&ray);
        }

        buffer[(i, j)] = Color::Rgb(pixel_color / (self.camera.sampling_rate as f64)).to_rgb_bytes();
      }
    }
    buffer
  }
  
  fn cast(&self,ray: &Ray) -> Option<HitRecord> {
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

  fn ray_color(&self, ray: &Ray) -> Vec3 {
    if let Some(hit) = self.cast(ray) {
      let n = hit.normal;
      return Vec3::new((n.x + 1.0) / 2.0, (n.y + 1.0) / 2.0, (n.z + 1.0) / 2.0);
    }
    let unit_direction = ray.dir.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.3, 0.5, 1.0)
  }

}
