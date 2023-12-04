mod event_listener;

use std::{io, panic};

use anyhow::Result;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture, KeyEvent},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{app::App, ui};

use self::event_listener::EventListener;

type Crossterm = Terminal<CrosstermBackend<io::Stderr>>;

#[derive(Clone, Debug)]
pub enum Event {
  KeyPress(KeyEvent),
  Resize,
  Error(String),
}

pub struct Tui {
  terminal: Crossterm,
  events: EventListener,
}

impl Tui {
  pub fn try_new() -> Result<Self> {
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventListener::new();
    Ok(Self { terminal, events })
  }

  pub fn enter(&mut self) -> Result<()> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(
      io::stderr(),
      EnterAlternateScreen,
      EnableMouseCapture
    )?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
      Self::reset().expect("failed to reset the terminal");
      panic_hook(panic);
    }));

    self.terminal.hide_cursor()?;
    self.terminal.clear()?;
    Ok(())
  }

  pub fn draw(&mut self, app: &App) -> Result<()> {
    self
      .terminal
      .draw(|frame| ui::render(&app.state(), frame))?;
    Ok(())
  }

  pub async fn event(&mut self) -> Result<Event> {
    self.events.next().await
  }

  fn reset() -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(
      io::stderr(),
      LeaveAlternateScreen,
      DisableMouseCapture
    )?;
    Ok(())
  }

  pub fn exit(&mut self) -> Result<()> {
    Self::reset()?;
    self.terminal.show_cursor()?;
    Ok(())
  }
}
