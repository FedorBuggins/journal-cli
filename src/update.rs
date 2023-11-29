use anyhow::Result;
use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};

use crate::{app::App, tui::event::Event};

pub fn handle_event(app: &mut App, event: Event) -> Result<()> {
  match event {
    Event::KeyPress(k_event) => handle_key_event(app, k_event),
    Event::Error(error) => return Err(anyhow::anyhow!(error)),
    _ => (),
  }
  Ok(())
}

fn handle_key_event(app: &mut App, k_event: KeyEvent) {
  match k_event.code {
    Left => app.prev_date(),
    Right => app.next_date(),
    Char('s') => app.add_smoke_record(),
    Char('u') => app.undo(),
    Esc | Char('q') => app.quit(),
    Char('c' | 'C') if k_event.modifiers == KeyModifiers::CONTROL => {
      app.quit()
    }
    _ => (),
  }
}
