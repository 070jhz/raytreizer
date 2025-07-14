use crate::{color::Color, math::Vec3, ray::Ray};
use crate::scene::HitRecord;

pub trait Material {
  fn albedo(&self) -> Color;
  fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Solid {
  pub albedo: Color,
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
    Some( (Ray::new(offset_origin, scatter_dir), self.albedo()) )
    
  }
}
