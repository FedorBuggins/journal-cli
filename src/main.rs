mod app;
mod event;
mod tui;
mod ui;
mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::prelude::{CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
  let mut app = App::default();
  let mut tui = Tui::new(
    Terminal::new(CrosstermBackend::new(std::io::stderr()))?,
    EventHandler::new(250),
  );

  tui.enter()?;

  while !app.should_quit() {
    tui.draw(&mut app)?;
    match tui.events.next()? {
      Event::Tick => {}
      Event::Key(k_event) => update(&mut app, k_event),
    };
  }

  tui.exit()?;

  Ok(())
}
