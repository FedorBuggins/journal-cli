mod app;
mod fs_journal;
mod tui;
mod ui;

use std::path::PathBuf;

use anyhow::Result;
use app::journal::Journal;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use tokio::sync::mpsc;

use self::{
  app::{App, Command},
  fs_journal::FsJournal,
  tui::Tui,
};

#[tokio::main]
async fn main() -> Result<()> {
  Tui::try_new()?.launch(&mut app().init()?).await
}

fn app() -> App {
  App::new([
    ("Trains", journal(home().join(".journals/trains"))),
    ("Smokes", journal(home().join(".journals/smokes"))),
  ])
}

fn home() -> PathBuf {
  PathBuf::from(env!("HOME"))
}

fn journal(dir: impl Into<PathBuf>) -> Box<dyn Journal> {
  Box::new(FsJournal::new(dir))
}

impl tui::App for App {
  fn render(&self, f: &mut Frame) {
    ui::render(&self.state(), f)
  }

  fn handle_key_event(&mut self, k_event: KeyEvent) -> Result<()> {
    use Command::*;
    use KeyCode::*;
    match k_event.code {
      Esc => self.handle_cmd(Quit),
      Tab => self.handle_cmd(NextTab),
      Up => self.handle_cmd(PrevSelection),
      Down => self.handle_cmd(NextSelection),
      Left => self.handle_cmd(PrevDate),
      Right => self.handle_cmd(NextDate),
      Char(' ') => self.handle_cmd(AddRecord),
      Backspace => self.handle_cmd(DeleteSelectedRecord),
      Char('u') => self.handle_cmd(Undo),
      Char('U') => self.handle_cmd(Redo),
      _ => Ok(()),
    }
  }

  fn changes(&mut self) -> &mut mpsc::Receiver<()> {
    self.changes()
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }
}
