use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub struct Body {
  pub tabs: Rect,
  pub date: Rect,
  pub days: Rect,
  pub time: Rect,
  pub year: Rect,
  pub help: Rect,
}

impl Body {
  pub fn new(size: Rect) -> Self {
    let [tabs_date, days_time, year, help] = destruct_layout(
      &Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Length(1),
          Constraint::Length(26),
          Constraint::Min(5),
          Constraint::Length(1),
        ])
        .split(size),
    );

    let [tabs, date] = destruct_layout(
      &Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
          Constraint::Percentage(60),
          Constraint::Percentage(40),
        ])
        .split(tabs_date),
    );

    let [days, _, time] = destruct_layout(
      &Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
          Constraint::Length(42),
          Constraint::Length(1),
          Constraint::Min(10),
        ])
        .split(days_time),
    );

    Self {
      tabs,
      date,
      days,
      time,
      year,
      help,
    }
  }
}

fn destruct_layout<const N: usize>(layout: &[Rect]) -> [Rect; N] {
  let mut i = 0;
  [0; N].map(|_| {
    let rect = layout[i];
    i += 1;
    rect
  })
}
