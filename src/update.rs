use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, k_event: KeyEvent) {
  use KeyCode::*;

  match k_event.code {
    Right | Char('j') => app.increment_counter(),
    Left | Char('k') => app.decrement_counter(),
    Esc | Char('q') => app.quit(),
    Char('c' | 'C') if k_event.modifiers == KeyModifiers::CONTROL => {
      app.quit()
    }
    _ => {}
  };
}
