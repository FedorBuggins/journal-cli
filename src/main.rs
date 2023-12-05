mod app;
mod journal;
mod tui;
mod ui;

use std::path::Path;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use self::{app::App, journal::Journal, tui::Tui};

const ROOT_DIR: &str = concat!(env!("HOME"), "/.journals");

#[tokio::main]
async fn main() -> Result<()> {
  let root_dir = Path::new(ROOT_DIR);
  let mut app = App::new([
    ("Trains", Journal::new(root_dir.join("trains"))),
    ("Smokes", Journal::new(root_dir.join("smokes"))),
  ]);
  Tui::try_new()?.launch(&mut app).await
}

impl tui::App for App {
  fn render(&self, f: &mut Frame) {
    ui::render(&self.state(), f)
  }

  fn handle_key_event(&mut self, k_event: KeyEvent) {
    use KeyCode::*;

    match k_event.code {
      Esc => self.quit(),
      Tab => self.next_tab(),
      Left => self.prev_date(),
      Right => self.next_date(),
      Char(' ') => self.add_record(),
      Char('u') => self.undo(),
      Char('U') => self.redo(),
      _ => (),
    }
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }

  fn quit(&mut self) {
    self.quit()
  }
}
