use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub struct Body {
  pub title: Rect,
  pub date: Rect,
  pub dates: Rect,
  pub time: Rect,
  pub year: Rect,
  pub help: Rect,
}

impl Body {
  pub fn new(size: Rect) -> Self {
    let [title, date, date_time, year, help] = destruct_layout(
      &Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Length(1),
          Constraint::Length(1),
          Constraint::Length(26),
          Constraint::Min(5),
          Constraint::Length(1),
        ])
        .split(size),
    );

    let [dates, time] = destruct_layout(
      &Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
          Constraint::Percentage(72),
          Constraint::Percentage(28),
        ])
        .split(date_time),
    );

    Self {
      title,
      date,
      dates,
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
