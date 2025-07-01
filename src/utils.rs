use rand::{thread_rng, Rng};


pub fn random_double() -> f64 {
  let mut rng = thread_rng();
  rng.r#gen()
}

pub fn random_double_in(min: f64, max: f64) -> f64 {
  min + (max - min) * random_double()
}
