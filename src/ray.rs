use crate::math::Vec3;
use crate::math::Point3;

pub struct Ray {
  pub o: Point3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Self {
    Ray { o: origin, dir: direction }
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.o + t * self.dir
  }
}
