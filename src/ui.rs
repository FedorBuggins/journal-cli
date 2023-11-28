mod layout;

use chrono::{Datelike, Local, Month, NaiveDate};
use ratatui::{
  prelude::{Alignment, Direction, Frame},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{
    Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding,
    Paragraph,
  },
};

use crate::app::State;

use self::layout::Body;

pub fn render(state: &State, f: &mut Frame) {
  let Body {
    title,
    date,
    dates,
    time,
    year,
    help,
  } = Body::new(f.size());

  f.render_widget(
    Paragraph::new("Smoker Journal")
      .alignment(Alignment::Center)
      .style(
        Style::default()
          .fg(Color::Yellow)
          .add_modifier(Modifier::BOLD),
      ),
    title,
  );

  f.render_widget(
    date_paragraph(state.date)
      .style(
        Style::default()
          .fg(Color::Green)
          .add_modifier(Modifier::BOLD),
      )
      .alignment(Alignment::Center),
    date,
  );

  f.render_widget(
    date_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Date")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    dates,
  );

  f.render_widget(
    time_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Time")
          .padding(Padding::horizontal(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    time,
  );

  f.render_widget(
    year_smoke_records_bar_chart(state)
      .block(
        Block::default()
          .title("Year")
          .padding(Padding::horizontal(4))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    year,
  );

  f.render_widget(
    help_paragraph()
      .style(Style::default().fg(Color::DarkGray))
      .alignment(Alignment::Center),
    help,
  );
}

fn date_paragraph<'a>(date: NaiveDate) -> Paragraph<'a> {
  Paragraph::new(format!("<- {} ->", date.format("%b %-d")))
}

fn help_paragraph<'a>() -> Paragraph<'a> {
  Paragraph::new("`s` - add record, `u` - undo, `q` - quit")
}

fn time_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  const HOURS_COUNT: u8 = 24;

  let bar_style = Style::default().fg(Color::Red);
  let bars: Vec<_> = (0..HOURS_COUNT)
    .map(|h| {
      let label = format!("{:0>2}:00", h);
      let value = state.date_smokes_by_hour.get(&h).map_or(0, |v| *v);
      Bar::default()
        .label(Line::raw(label))
        .value(value as _)
        .text_value(String::new())
        .style(bar_style)
    })
    .collect();

  BarChart::default()
    .direction(Direction::Horizontal)
    .bar_gap(0)
    .max(3)
    .data(BarGroup::default().bars(&bars))
}

fn date_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  let bars: Vec<_> = state
    .recently_dates_smokes_count
    .iter()
    .map(|(date, smokes_count)| {
      let label = date.format("%d").to_string();
      let value = *smokes_count;
      let style = date_style(date, state);
      Bar::default()
        .label(Line::styled(label, style))
        .value(value as _)
        .style(style)
    })
    .collect();

  BarChart::default()
    .bar_width(2)
    .bar_gap(2)
    .data(BarGroup::default().bars(&bars))
}

fn date_style(date: &NaiveDate, state: &State) -> Style {
  Style::default().fg(if date == &state.date {
    Color::Green
  } else {
    Color::Yellow
  })
}

fn year_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  let today = Local::now().date_naive();
  let cur_month = Month::try_from(today.month0() as u8 + 1).unwrap();
  let past_year = iter_months(cur_month).skip(1).take(12);
  let bars: Vec<_> = past_year
    .map(|month| {
      let label = month.name()[0..3].to_string();
      let value =
        state.year_smokes_by_month.get(&month).map_or(0, |v| *v);
      Bar::default().label(Line::raw(label)).value(value as _)
    })
    .collect();

  BarChart::default()
    .bar_width(3)
    .bar_gap(1)
    .data(BarGroup::default().bars(&bars))
}

fn iter_months(start: Month) -> impl Iterator<Item = Month> {
  (0..).scan(start, |next, _| {
    let cur = *next;
    *next = cur.succ();
    Some(cur)
  })
}
