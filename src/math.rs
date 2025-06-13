use std::ops::{Add, Sub, Neg, Index, IndexMut};

#[derive(Clone, Copy)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl std::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, i: usize) -> &Self::Output {
    match i {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of bounds for Vec3"),
    }
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, i :usize) -> &mut Self::Output {
    match i {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Index out of bounds for Vec3"),
    }
  }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
  type Output = Vec3;

  fn add(self, rhs: &'b Vec3) -> Vec3 {
    Vec3::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z,
    )
  }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: &'b Vec3) -> Self::Output {
    Vec3::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z,     
    )
  }
}

impl Vec3 {

  pub fn new(x: f64, y: f64, z:f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn zero() -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn x(&self) -> f64 { self.x }
  pub fn y(&self) -> f64 { self.y }
  pub fn z(&self) -> f64 { self.z }
  
  pub fn length(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

}
