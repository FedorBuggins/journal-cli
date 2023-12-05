mod event_listener;

use std::{io, panic};

use anyhow::Result;
use crossterm::{
  event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent,
    KeyModifiers,
  },
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};

use self::event_listener::EventListener;

type Crossterm = Terminal<CrosstermBackend<io::Stderr>>;

#[derive(Clone, Debug)]
pub enum Event {
  KeyPress(KeyEvent),
  Resize,
  Error(String),
}

pub trait App {
  fn render(&self, f: &mut Frame);
  fn handle_key_event(&mut self, k_event: KeyEvent) -> Result<()>;
  fn should_quit(&self) -> bool;
  fn quit(&mut self);
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

  pub async fn launch(mut self, app: &mut impl App) -> Result<()> {
    self.enter()?;
    let err = self.run(app).await.err();
    self.exit()?;
    if let Some(err) = err {
      eprintln!("{err}");
    }
    Ok(())
  }

  fn enter(&mut self) -> Result<()> {
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

  fn reset() -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(
      io::stderr(),
      LeaveAlternateScreen,
      DisableMouseCapture
    )?;
    Ok(())
  }

  async fn run(&mut self, app: &mut impl App) -> Result<()> {
    while !app.should_quit() {
      self.terminal.draw(|f| app.render(f))?;
      match self.events.next().await? {
        Event::Resize => (),
        Event::Error(error) => return Err(anyhow::anyhow!(error)),
        Event::KeyPress(k_event) => match k_event.code {
          KeyCode::Char('c' | 'C') if is_ctrl(k_event) => app.quit(),
          _ => app.handle_key_event(k_event)?,
        },
      }
    }
    Ok(())
  }

  fn exit(&mut self) -> Result<()> {
    Self::reset()?;
    self.terminal.show_cursor()?;
    Ok(())
  }
}

fn is_ctrl(k_event: KeyEvent) -> bool {
  k_event.modifiers == KeyModifiers::CONTROL
}
