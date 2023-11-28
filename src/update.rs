use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};

use crate::{app::App, event::Event};

pub fn handle_event(app: &mut App, event: Event) {
  match event {
    Event::KeyPress(k_event) => handle_key_event(app, k_event),
    Event::Error(error) => {
      eprintln!("{error}");
      app.quit();
    }
    _ => (),
  }
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
