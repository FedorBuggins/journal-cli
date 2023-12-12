#[derive(Default, Clone, PartialEq)]
pub struct Level {
  count: usize,
  middle: f32,
}

impl Level {
  pub fn new(count: usize, middle: f32) -> Self {
    Self { count, middle }
  }

  pub fn target(&self) -> usize {
    self.middle as _
  }

  pub fn count(&self) -> usize {
    self.count
  }

  pub fn percentage(&self) -> f32 {
    self.count as f32 / self.middle
  }

  pub fn middle(&self) -> f32 {
    self.middle
  }
}
