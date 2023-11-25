#[derive(Default)]
pub struct App {
  counter: i64,
  should_quit: bool,
}

impl App {
  pub fn counter(&self) -> i64 {
    self.counter
  }

  pub fn increment_counter(&mut self) {
    self.counter += 1;
  }

  pub fn decrement_counter(&mut self) {
    self.counter -= 1;
  }

  pub fn should_quit(&self) -> bool {
    self.should_quit
  }

  pub fn quit(&mut self) {
    self.should_quit = true;
  }
}
