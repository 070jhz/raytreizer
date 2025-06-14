use crate::math::Vec3;

pub enum Color {
  Rgb(Vec3),
  Argb(f64, Vec3),
}

impl Color {
  pub fn rgb(r: f64, g: f64, b: f64) -> Self {
    assert!((0.0..=1.0).contains(&r), "Red out of range: {r}");
    assert!((0.0..=1.0).contains(&g), "Green out of range: {g}");
    assert!((0.0..=1.0).contains(&b), "Blue out of range: {b}");
    Color::Rgb(Vec3::new(r, g, b))
  }

  pub fn argb(a: f64, r: f64, g: f64, b:f64) -> Self {
    Color::Argb(a, Vec3::new(r, g, b))
  }
  
  // top byte is ignored because alpha blending isnt supported by minifb
  pub fn to_rgb_bytes(&self) -> u32 {
    match *self {
      Color::Rgb(v) => {
        let (r, g, b) = (
          (v.x * 255.0) as u32,
          (v.y * 255.0) as u32,
          (v.z * 255.0) as u32
        );
        
        (r << 16) | (g << 8) | b
      }
      Color::Argb(a,v) => {
        let (r, g, b) = (
          (v.x * 255.0) as u32,
          (v.y * 255.0) as u32,
          (v.z * 255.0) as u32
        );
        let alpha = (a * 255.0) as u32;
        (alpha << 24) | (r << 16) | (g << 8) | b
      }
    }
  }

  pub fn alpha(&self) -> f64 {
    match *self {
      Color::Rgb(_) => 1.0,
      Color::Argb(a, _) => a,
    }
  }
}
