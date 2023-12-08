mod app;
mod fs_journal;
mod tui;
mod ui;

use std::path::Path;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use self::{
  app::{App, Command},
  fs_journal::FsJournal,
  tui::Tui,
};

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
    use Command::*;
    use KeyCode::*;

    self.handle_cmd(match k_event.code {
      Esc => Quit,
      Tab => NextTab,
      Up => PrevSelection,
      Down => NextSelection,
      Left => PrevDate,
      Right => NextDate,
      Char(' ') => AddRecord,
      Backspace => DeleteSelectedRecord,
      Char('u') => Undo,
      Char('U') => Redo,
      _ => Unknown,
    })
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }

  fn quit(&mut self) {
    self.quit()
  }
}
