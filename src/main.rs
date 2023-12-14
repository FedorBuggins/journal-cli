mod app;
mod fs_journal;
mod tui;
mod ui;

use std::path::PathBuf;

use anyhow::Result;
use app::journal::Journal;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use self::{
  app::{App, Command},
  fs_journal::FsJournal,
  tui::Tui,
};

fn main() -> Result<()> {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .build()?
    .block_on(async {
      Tui::try_new()?.launch(&mut app().init().await?).await
    })
}

fn app() -> App {
  App::new([
    ("Trains", 4, journal(home().join(".journals/trains"))),
    ("Smokes", 1, journal(home().join(".journals/smokes"))),
  ])
}

fn home() -> PathBuf {
  PathBuf::from(env!("HOME"))
}

fn journal(dir: impl Into<PathBuf>) -> Box<dyn Journal> {
  Box::new(FsJournal::new(dir))
}

#[async_trait]
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

  async fn changed(&mut self) {
    self.changed().await
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }
}
