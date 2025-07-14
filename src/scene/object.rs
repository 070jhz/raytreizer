use std::f64;
use std::sync::Arc;

use crate::Vec3;
use crate::Point3;
use crate::Ray;
use crate::scene::material::Material;

const EPSILON: f64 = 1e-8;

pub struct HitRecord<'a> {
  pub t       : f64,
  pub point   : Point3,
  pub normal  : Vec3,
  pub material: &'a dyn Material,
}

pub trait Hittable {
  fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

// Object enum and variants

pub enum Object {
  Sphere(Sphere),
  Plane(Plane),
  Cylinder(Cylinder),
}

pub struct Sphere {
  pub center  : Point3,
  pub radius  : f64,
  pub material: Arc<dyn Material + Send + Sync>,
}

pub struct Plane {
  pub anchor  : Point3,
  pub normal  : Vec3,
  pub material: Arc<dyn Material + Sync + Send>,
}

pub struct Cylinder {
  pub center: Point3,
  pub radius: f64,
  pub height: f64,
  pub orientation: Vec3,
  pub body_material: Arc<dyn Material + Sync + Send>,
  pub top_material: Arc<dyn Material + Sync + Send>,
  pub bottom_material: Arc<dyn Material + Sync + Send>,
}

// intersection implementations

impl Hittable for Object {
  fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    match self {
      Object::Sphere(s)   => s.hit(ray),
      Object::Plane(p)    => p.hit(ray),
      Object::Cylinder(c) => c.hit(ray),
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    // direction, origin and vector from ray to center
    let oc        = self.center - ray.o;
    let dir         = ray.dir;
    
    // quadratic coefficients and discriminant
    let a = dir.length_squared();
    let h = dir.dot(&oc);
    let c = oc.dot(&oc) - self.radius * self.radius;
    let d = h*h - a*c;

    if d < 0.0 {
      return None;
    }

    let d_sqrt = d.sqrt();
    let t1 = (h - d_sqrt) / a;
    
    // return early when the closest root is in front of the camera
    if t1 >= EPSILON {
      let point: Point3 = ray.at(t1);
      let normal = ((point - self.center) / self.radius).unit();

      return Some(HitRecord {
        t: t1,
        point,
        normal,
        material: self.material.as_ref(),
      });
    }

    // if d == 0.0 equation yields the same root twice
    let t2 = (h + d_sqrt) / a;
    // check the second root
    if t2 >= EPSILON {
      let point : Point3 = ray.at(t2);
      let normal = ((point - self.center) / self.radius).unit();

      return Some(HitRecord {
        t: t2,
        point,
        normal,
        material: self.material.as_ref(),
      });
    }

    None
  }

}

impl Hittable for Plane {
  fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    let denominator = ray.dir.dot(&self.normal);
    
    if denominator.abs() < EPSILON {
      // ray is parallel to the plane
      return None;
    }

    let t = (self.anchor - ray.o).dot(&self.normal) / denominator;
    if t < EPSILON {
      return None;
    } else {
      let point : Point3 = ray.at(t);

      Some(HitRecord {
        t,
        point,
        normal: self.normal,
        material: self.material.as_ref(),
      })
    }
  }
}

impl Hittable for Cylinder {
  fn hit(&self, ray: &Ray) -> Option<HitRecord> {
    // to factor out orientation and position of the cylinder,
    // we use projection math to determine whether the ray intersects 
    let oc  = ray.o - self.center;
    let dir = ray.dir;
    let v   = self.orientation;
    
    // ray direction component perpendicular to cylinder axis (transverse direction)
    let n = dir - dir.dot(&v) * v;
    // oc component perpendicular to cylinder axis (radial offset)
    let m = oc - oc.dot(&v) * v;
    
    let mut closest_hit: Option<HitRecord> = None;
    let mut closest_t = f64::INFINITY;

    // reduced coefficients and discriminant
    let a = n.length_squared();
    if a <= EPSILON {
      return None; // ray is parallel to cylinder axis -> no side intersection
    }

    let h = n.dot(&m);
    let c = m.dot(&m) - self.radius * self.radius;
    let d = h*h - a*c;
    
    if d < 0.0 {
      return None;
    }
    
    let d_sqrt = d.sqrt();
    
    for t in [(-h - d_sqrt) / a, (-h + d_sqrt) / a] {
      if t >= EPSILON {
        let point = ray.at(t);
        let height = (point - self.center).dot(&v);

        if height >= 0.0 && height <= self.height {
          let axis_point = self.center + height * v;
          let normal = (point - axis_point).unit();

          if t < closest_t {
            closest_t = t;
            closest_hit = Some(HitRecord {
              t,
              point,
              normal,
              material: self.body_material.as_ref(),
            })
          }
        }
      }
    }

    // check cylinder caps

    let top = Plane {
      anchor  : self.center + self.height * v,
      normal  : v,
      material: Arc::clone(&self.top_material),
    };

    let bottom = Plane {
      anchor  : self.center,
      normal  : -v,
      material: Arc::clone(&self.bottom_material),
    };

    
    for plane in &[bottom, top] {
      if let Some(hit) = plane.hit(ray) {
        let dist2 = (hit.point - plane.anchor).length_squared();
        if dist2 <= self.radius * self.radius && hit.t < closest_t {
          closest_t   = hit.t;
          closest_hit = Some(HitRecord {
            t      : hit.t,
            point  : hit.point,
            normal : plane.normal,
            material: if plane.normal == v {
              self.top_material.as_ref()
            } else {
              self.bottom_material.as_ref()
            },
          }); 
        }
      }
    }

    closest_hit

  }
}
