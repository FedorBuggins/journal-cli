use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};

use crate::{app::App, event::Event};

pub fn handle_event(app: &mut App, event: Event) {
  if let Event::Key(k_event) = event {
    handle_key_event(app, k_event)
  }
}

fn handle_key_event(app: &mut App, k_event: KeyEvent) {
  match k_event.code {
    Left => app.prev_date(),
    Right => app.next_date(),
    Char('s') => app.add_smoke_record(),
    Esc | Char('q') => app.quit(),
    Char('c' | 'C') if k_event.modifiers == KeyModifiers::CONTROL => {
      app.quit()
    }
    _ => (),
  }
}
