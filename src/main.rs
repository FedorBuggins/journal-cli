mod app;
mod fs_journal;
mod tui;
mod ui;

use std::path::Path;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use self::{app::App, fs_journal::FsJournal, tui::Tui};

const ROOT_DIR: &str = concat!(env!("HOME"), "/.journals");

#[tokio::main]
async fn main() -> Result<()> {
  let root_dir = Path::new(ROOT_DIR);
  let mut app = App::try_new(vec![
    ("Trains", Box::new(FsJournal::new(root_dir.join("trains")))),
    ("Smokes", Box::new(FsJournal::new(root_dir.join("smokes")))),
  ])?;
  Tui::try_new()?.launch(&mut app).await
}

impl tui::App for App {
  fn render(&self, f: &mut Frame) {
    ui::render(&self.state(), f)
  }

  fn handle_key_event(&mut self, k_event: KeyEvent) -> Result<()> {
    use KeyCode::*;

    match k_event.code {
      Esc => self.quit(),
      Tab => self.next_tab()?,
      Up => self.prev_record()?,
      Down => self.next_record()?,
      Left => self.prev_date()?,
      Right => self.next_date()?,
      Char(' ') => self.add_record()?,
      Backspace => self.delete_selected_record()?,
      Char('u') => self.undo()?,
      Char('U') => self.redo()?,
      _ => (),
    }
    Ok(())
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }

  fn quit(&mut self) {
    self.quit()
  }
}
