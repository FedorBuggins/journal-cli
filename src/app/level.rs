#[derive(Default, Clone, PartialEq)]
pub struct Level {
  count: usize,
  middle: f32,
  target: usize,
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
impl Level {
  pub fn new(count: usize, middle: f32, target: usize) -> Self {
    Self {
      count,
      middle,
      target,
    }
  }

  pub fn target(&self) -> usize {
    self._target().round() as _
  }

  fn _target(&self) -> f32 {
    let new_middle = (self.middle + self.target as f32) / 2.;
    new_middle.max(self.middle * 0.8)
  }

  pub fn count(&self) -> usize {
    self.count
  }

  pub fn percentage(&self) -> f32 {
    self.count as f32 / self._target()
  }

  pub fn is_positive(&self) -> bool {
    self.target as f32 > self.middle
  }

  pub fn for_count(&self, count: usize) -> Self {
    Self { count, ..*self }
  }
}
