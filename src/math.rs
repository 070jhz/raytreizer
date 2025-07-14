use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::random_double_in;

#[derive(Clone, Copy)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

// Vec3 implementation
impl Vec3 {

  pub fn new(x: f64, y: f64, z:f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn zero() -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }
  
  pub fn length(&self) -> f64 {
    return self.length_squared().sqrt();
  }
  
  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn dot(&self, other: &Vec3) -> f64 {
    self.x * other.x 
    + self.y * other.y
    + self.z * other.z
  }

  pub fn cross(&self, other: &Vec3) -> Vec3 {
    Vec3::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    )
  }

  pub fn unit(&self) -> Vec3 {
    *self / self.length()
  }

  pub fn random_unit_sphere() -> Vec3 {
    loop {
      let v = Vec3::new(
        random_double_in(-1.0, 1.0),
        random_double_in(-1.0, 1.0),
        random_double_in(-1.0, 1.0)
      );

      if v.length_squared() < 1.0 {
        return v.unit();
      }
    }  
  }
}

pub type Point3 = Vec3;

// for printing
impl std::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
  }
}

// negation
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

// indexing and indexing mutably
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

// vector addition 
impl Add<Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Vec3) -> Vec3 {
    Vec3::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z,
    )
  }
}

// in-place vector addition
impl AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

// vector subtraction
impl Sub<Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Vec3) -> Self::Output {
    Vec3::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z,     
    )
  }
}

// in-place vector subtraction
impl SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

// hadamard product
impl Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(
      self.x * rhs.x,
      self.y * rhs.y,
      self.z * rhs.z,
    )
  }
}

// in-place hadamard product
impl MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
  }
}

// scalar product
impl Mul<f64> for Vec3 {
  type Output = Vec3;
  
  fn mul(self, scalar: f64) -> Self::Output {
    Vec3::new(
      self.x * scalar,
      self.y * scalar,
      self.z * scalar,
    )
  }
}

// for commutativity
impl Mul<Vec3> for f64 {
  type Output = Vec3;
  
  fn mul(self, v: Vec3) -> Self::Output {
    v * self
  }
}

// in-place scalar product
impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, scalar: f64) {
    self.x *= scalar;
    self.y *= scalar;
    self.z *= scalar;
  }
}

// scalar division
impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, scalar: f64) -> Self::Output {
    assert!(scalar != 0.0, "Cannot divide by zero scalar");
    self * (1.0 / scalar)    
  }
}

// in-place scalar division
impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, scalar: f64) {
    *self *= 1.0 / scalar
  }
}

