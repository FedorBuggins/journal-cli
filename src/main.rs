mod app;
mod fs_journal;
mod tui;
mod ui;

use anyhow::Result;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use self::{
  app::{App, Command},
  tui::Tui,
};

mod cfg {
  use std::path::PathBuf;

  use crate::{app::journal::Journal, fs_journal::FsJournal};

  pub struct Tab {
    pub title: &'static str,
    pub target: usize,
  }

  pub fn tabs() -> Vec<Tab> {
    [
      Tab {
        title: "Smokes",
        target: 1,
      },
      Tab {
        title: "Trains",
        target: 4,
      },
    ]
    .into()
  }

  pub fn journals_dir() -> PathBuf {
    PathBuf::from(env!("HOME")).join(".journals")
  }

  pub fn journal(dir: impl Into<PathBuf>) -> Box<dyn Journal> {
    Box::new(FsJournal::new(dir))
  }
}

fn main() -> Result<()> {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    .build()?
    .block_on(launch())
}

async fn launch() -> Result<()> {
  Tui::try_new()?.launch(&mut app().init().await?).await?;
  Ok(())
}

fn app() -> App {
  App::new(cfg::tabs().into_iter().map(
    |cfg::Tab { title, target }| {
      let journal =
        cfg::journal(cfg::journals_dir().join(title.to_lowercase()));
      (title, target, journal)
    },
  ))
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
