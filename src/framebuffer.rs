use std::ops::{ Index, IndexMut };

pub struct FrameBuffer {
  pub width: usize,
  pub height: usize,
  pub buf: Vec<u32>,
}

impl FrameBuffer {
  pub fn new(width:usize, height: usize) -> Self {
    FrameBuffer {
      width,
      height,
      buf: vec![0; width * height],
    }
  }
}

impl Index<(usize, usize)> for FrameBuffer {
  type Output = u32;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (x, y) = index;
    &self.buf[y * self.width + x]
  }
}

impl IndexMut<(usize, usize)> for FrameBuffer {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (x, y) = index;
    &mut self.buf[y * self.width + x]
  }
}

