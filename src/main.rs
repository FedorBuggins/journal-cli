mod app;
mod event;
mod journal;
mod tui;
mod ui;
mod update;

use anyhow::Result;
use app::App;
use event::EventHandler;
use ratatui::prelude::{CrosstermBackend, Terminal};
use tui::Tui;
use update::handle_event;

fn main() -> Result<()> {
  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let mut tui = Tui::new(terminal, EventHandler::new(250));
  tui.enter()?;
  run(&mut tui)?;
  tui.exit()?;
  Ok(())
}

fn run(tui: &mut Tui) -> Result<()> {
  let mut app = App::default();
  while !app.should_quit() {
    tui.draw(&app)?;
    handle_event(&mut app, tui.events.next()?);
  }
  Ok(())
}
