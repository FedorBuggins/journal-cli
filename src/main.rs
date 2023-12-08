mod app;
mod fs_journal;
mod tui;
mod ui;

use std::path::Path;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use tokio::sync::mpsc;

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

  fn changes(&mut self) -> &mut mpsc::Receiver<()> {
    self.changes()
  }

  fn should_quit(&self) -> bool {
    self.should_quit()
  }
}

#[cfg(test)]
mod learning_tests {
  use chrono::{Duration, NaiveTime};
  use tokio::{sync::mpsc, time::sleep};

  #[test]
  fn time_add_dutation_with_overflow() {
    let time = NaiveTime::from_hms_opt(23, 0, 0).unwrap();
    let result = time + Duration::hours(1);
    let expected = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    assert_eq!(result, expected);
  }

  #[tokio::test]
  async fn channel_hold_all_messages_in_queue() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    tx.send(0).unwrap();
    tick().await;
    tx.send(1).unwrap();
    tick().await;
    let r0 = rx.try_recv();
    tick().await;
    let r1 = rx.recv().await;
    assert_eq!(Ok(0), r0);
    assert_eq!(Some(1), r1);
  }

  async fn tick() {
    sleep(Duration::milliseconds(100).to_std().unwrap()).await;
  }
}
