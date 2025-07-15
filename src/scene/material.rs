use crate::{color::Color, math::{Vec3, EPSILON}, ray::Ray};
use crate::scene::HitRecord;

pub trait Material {
  fn albedo(&self) -> Color;
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Solid {
  pub albedo: Color,
}

pub struct BasicMetal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Material for Solid {
  fn albedo(&self) -> Color {
      self.albedo
  }

  fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    let mut scatter_dir = rec.normal + Vec3::random_unit_sphere();
    
    if scatter_dir.dot(&rec.normal) < 0.0 {
      scatter_dir = -scatter_dir;
    }
    
    let offset_origin = rec.point + rec.normal * 1e-4;
    Some( (Ray::new(offset_origin, scatter_dir.unit()), self.albedo()) )
    
  }
}

impl Material for BasicMetal {
  fn albedo(&self) -> Color {
      self.albedo
  }

  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    let reflected_dir = ray.dir - 2.0 * ray.dir.dot(&rec.normal) * rec.normal;    
    
    if reflected_dir.dot(&rec.normal) < EPSILON {
      return None;
    }

    let mut random_offset = Vec3::random_unit_sphere();
    if random_offset.dot(&rec.normal) < 0.0 {
      random_offset= -random_offset;
    }

    let fuzzed = reflected_dir + (self.fuzz * random_offset);
    if fuzzed.length_squared() < EPSILON * EPSILON {
      return None;
    }

    let offset_origin = rec.point + rec.normal * EPSILON;
    Some( (Ray::new(offset_origin, fuzzed.unit()), self.albedo()) )
  }
}
