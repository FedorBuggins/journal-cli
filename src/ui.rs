use std::collections::HashMap;

use chrono::{Days, Local, NaiveDate, Timelike};
use ratatui::{
  prelude::{Alignment, Constraint, Direction, Frame, Layout, Rect},
  style::{Color, Modifier, Style},
  text::Line,
  widgets::{
    Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding,
    Paragraph,
  },
};

use crate::app::App;

pub fn render(app: &App, f: &mut Frame) {
  let (header, date, dates, time) = layout(f);

  f.render_widget(
    help_paragraph()
      .block(
        Block::default()
          .title("Smoke Journal")
          .title_alignment(Alignment::Center)
          .title_style(
            Style::default()
              .fg(Color::Yellow)
              .add_modifier(Modifier::BOLD),
          )
          .padding(Padding::uniform(1)),
      )
      .style(Style::default().fg(Color::DarkGray))
      .alignment(Alignment::Center),
    header,
  );

  f.render_widget(
    date_paragraph(app.date())
      .style(
        Style::default()
          .fg(Color::Green)
          .add_modifier(Modifier::BOLD),
      )
      .alignment(Alignment::Center),
    date,
  );

  f.render_widget(
    date_smoke_records_bar_chart(app)
      .block(
        Block::default()
          .title("Date")
          .padding(Padding::uniform(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    dates,
  );

  f.render_widget(
    time_smoke_records_bar_chart(app)
      .block(
        Block::default()
          .title("Time")
          .padding(Padding::uniform(1))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .style(Style::default().fg(Color::Yellow)),
    time,
  );
}

fn layout(f: &Frame<'_>) -> (Rect, Rect, Rect, Rect) {
  let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(4),
      Constraint::Length(2),
      Constraint::Length(28),
      Constraint::Length(0),
    ])
    .split(f.size());

  let [header, date, other] = [0, 1, 2].map(|i| layout[i]);

  let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(72),
      Constraint::Percentage(28),
    ])
    .split(other);

  let [dates, time] = [0, 1].map(|i| layout[i]);

  (header, date, dates, time)
}

fn date_paragraph<'a>(date: NaiveDate) -> Paragraph<'a> {
  Paragraph::new(format!("<- {} ->", date.format("%b %-d")))
}

fn help_paragraph<'a>() -> Paragraph<'a> {
  Paragraph::new(
    "Press `s` to add smoke record, `q` to stop running.",
  )
}

fn time_smoke_records_bar_chart(app: &App) -> BarChart<'_> {
  let rec_map = app.date_smoke_records().into_iter().fold(
    HashMap::new(),
    |mut map, dt| {
      *map.entry(dt.time().hour()).or_default() += 1;
      map
    },
  );

  BarChart::default()
    .direction(Direction::Horizontal)
    .bar_gap(0)
    .max(2)
    .data(
      BarGroup::default().bars(
        &(0..24)
          .map(|h| {
            Bar::default()
              .label(Line::raw(format!("{:0>2}:00", h)))
              .value(rec_map.get(&h).copied().unwrap_or_default())
              .text_value(String::new())
              .style(Style::default().fg(Color::Red))
          })
          .collect::<Vec<_>>(),
      ),
    )
}

fn date_smoke_records_bar_chart(app: &App) -> BarChart<'_> {
  let today = Local::now().date_naive();
  BarChart::default().bar_width(2).bar_gap(2).data(
    BarGroup::default().bars(
      &app
        .smoke_records_for(today - Days::new(9), today)
        .map(|(date, recs)| {
          let color = if date == app.date() {
            Color::Green
          } else {
            Color::Yellow
          };

          Bar::default()
            .label(Line::raw(date.format("%d").to_string()))
            .value(recs.len() as _)
            .style(Style::default().fg(color))
        })
        .collect::<Vec<_>>(),
    ),
  )
}
