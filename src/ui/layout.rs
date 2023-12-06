use ratatui::prelude::{Constraint, Direction, Layout, Rect};

pub struct Body {
  pub tabs: Rect,
  pub date: Rect,
  pub list: Rect,
  pub level: Rect,
  pub days: Rect,
  pub time: Rect,
  pub year: Rect,
  pub help: Rect,
}

impl Body {
  pub fn new(size: Rect) -> Self {
    use Constraint::*;

    let [tabs, date_list_level_days_time, year, help] =
      vsplit([Length(2), Length(26), Min(5), Length(1)], size);

    let [date_list_level_days, _, time] = hsplit(
      [Length(42), Length(1), Min(10)],
      date_list_level_days_time,
    );

    let [date_list_level, days] =
      vsplit([Percentage(50), Percentage(50)], date_list_level_days);

    let [date_list, _, level] =
      hsplit([Length(24), Length(1), Min(17)], date_list_level);

    let [date, list] = vsplit([Length(3), Min(1)], date_list);

    Self {
      tabs,
      date,
      list,
      level,
      days,
      time,
      year,
      help,
    }
  }
}

pub fn vsplit<const N: usize>(
  constraints: [Constraint; N],
  size: Rect,
) -> [Rect; N] {
  destruct_layout(
    &Layout::default()
      .direction(Direction::Vertical)
      .constraints(constraints)
      .split(size),
  )
}

pub fn hsplit<const N: usize>(
  constraints: [Constraint; N],
  size: Rect,
) -> [Rect; N] {
  destruct_layout(
    &Layout::default()
      .direction(Direction::Horizontal)
      .constraints(constraints)
      .split(size),
  )
}

fn destruct_layout<const N: usize>(layout: &[Rect]) -> [Rect; N] {
  let mut i = 0;
  [0; N].map(|_| {
    let rect = layout[i];
    i += 1;
    rect
  })
}
