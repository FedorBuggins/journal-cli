#[derive(Default, Clone)]
pub struct Level {
  pub percentage: f32,
  pub middle: f32,
}

impl Level {
  pub fn new(percentage: f32, middle: f32) -> Self {
    Self { percentage, middle }
  }

  pub fn target(&self) -> usize {
    self.middle as _
  }

  pub fn count(&self) -> usize {
    (self.middle * self.percentage) as _
  }
}
