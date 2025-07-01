use crate::Point3;
use crate::Vec3;

#[derive(Clone)]
pub struct Viewport {
  pub u       : Vec3,
  pub v       : Vec3,
  pub origin  : Point3, // top-left corner
  pub pdu                 : Vec3,     // pixel spacing in u direction
  pub pdv                 : Vec3,     // pixel spacing in v direction
  pub p00                 : Point3,   // location of top left pixel
  
}

#[derive(Clone)]
pub struct Camera {
  pub aspect_ratio    : f64,
  pub image_width     : usize,
  pub position        : Point3,
  pub focal_length    : f64,
  pub viewport        : Viewport,
  pub image_height    : usize,
  pub sampling_rate   : usize,
}

impl Camera {
  pub fn new(position: Point3,
    fov_degrees: f64,
    aspect_ratio: f64,
    image_width: usize) -> Self {

    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = image_height.max(1); 
    let actual_ratio = image_width as f64 / image_height as f64;
    
    let focal_length = 1.0;
    let viewport_height = 2.0 * focal_length * (fov_degrees.to_radians() / 2.0).tan();
    let viewport_width = viewport_height * aspect_ratio;
    
    // viewport edge vectors
    // Y-axis is inverted relatively to the traversal of the viewport (top-to-bottom)
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    let viewport_top_left = position
                          - Vec3::new(0.0, 0.0, focal_length)
                          - viewport_u / 2.0
                          - viewport_v / 2.0;

    let pdu         = viewport_u / (image_width as f64);
    let pdv         = viewport_v / (image_height as f64);
    let p00: Point3 = viewport_top_left + pdu / 2.0 + pdv / 2.0;

    let viewport = Viewport {
      u: viewport_u,
      v: viewport_v,
      origin: viewport_top_left,
      pdu,
      pdv,
      p00,
    };
      

    Camera {
      aspect_ratio  : actual_ratio,
      image_width,
      position,
      focal_length  : 1.0,
      viewport,
      image_height,
      sampling_rate : 4, 
    }
  }

  } 
