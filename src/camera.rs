use crate::Point3;
use crate::Vec3;

pub struct Viewport {
  pub u       : Vec3,
  pub v       : Vec3,
  pub origin  : Point3, // top-left corner
  pub width   : f64,    // derived from aspect ratio
  pub height  : f64,    // derived from fov
}

pub struct Camera {
  pub position        : Point3,
  pub focal_length    : f64,
  pub fov_degrees     : f64,    // vertical field of view
  pub viewport        : Viewport,
}

impl Camera {
  pub fn new(position: Point3, fov_degrees: f64, aspect_ratio: f64) -> Self {
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

    let viewport = Viewport {
      u: viewport_u,
      v: viewport_v,
      origin: viewport_top_left,
      width: viewport_width,
      height: viewport_height,
    };

    Camera {
      position,
      focal_length : 1.0,
      fov_degrees,
      viewport,
    }
  }
}
