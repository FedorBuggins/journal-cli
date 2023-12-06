use std::cmp::max;

use chrono::{Datelike, Local, Month, NaiveDate, Timelike};
use ratatui::{
  prelude::Direction,
  style::Stylize,
  text::Line,
  widgets::{
    Bar, BarChart, BarGroup, List, ListItem, Paragraph, Tabs,
  },
};

use crate::app::State;

use super::styles;

pub fn tabs(state: &State) -> Tabs<'_> {
  let titles = state
    .tabs
    .iter()
    .map(|title| format!("[ {title} ]"))
    .collect();
  Tabs::new(titles).divider("").select(state.tabs.selected())
}

pub fn date_paragraph<'a>(date: NaiveDate) -> Paragraph<'a> {
  Paragraph::new(format!("<- {} ->", date.format("%a, %-d %b %Y")))
}

pub fn record_list(state: &State) -> List<'_> {
  let items: Vec<_> = state
    .list
    .iter()
    .enumerate()
    .map(|(i, dt)| {
      let text = format!("{}) {}", i + 1, dt.format("%R"));
      ListItem::new(text).style(if i == state.list.selected() {
        styles::ACCENT.reversed()
      } else {
        styles::PRIMARY
      })
    })
    .collect();
  List::new(items)
}

pub fn date_records_bar_chart(state: &State) -> BarChart<'_> {
  let bars: Vec<_> = state
    .recs_by_date
    .iter()
    .map(|(date, count)| {
      use styles::*;
      let label = date.format("%d").to_string();
      let style = if date == &state.date { ACCENT } else { PRIMARY };
      Bar::default()
        .label(Line::styled(label, style))
        .value(*count as _)
        .style(style)
    })
    .collect();

  let max_val = state
    .recs_by_date
    .iter()
    .map(|(_, count)| count)
    .max()
    .map_or(0, |v| *v);

  BarChart::default()
    .bar_width(2)
    .bar_gap(2)
    .max(bar_max(max_val as _))
    .data(BarGroup::default().bars(&bars))
}

fn bar_max(max_val: u64) -> u64 {
  const INCREASE_PERCENTAGE: u64 = 10;
  max_val + max(max_val / INCREASE_PERCENTAGE, 1)
}

pub fn time_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  const HOURS_COUNT: u8 = 24;

  let bars: Vec<_> = (0..HOURS_COUNT)
    .map(|h| {
      let label = format!("{:0>2}:00", h);
      let value = state.recs_by_hour.get(&h).map_or(0, |v| *v);
      Bar::default()
        .label(label.into())
        .value(value as _)
        .text_value(String::new())
        .style(
          if state
            .list
            .selected_item()
            .is_some_and(|dt| h == dt.time().hour() as _)
          {
            styles::ACCENT
          } else {
            styles::RED
          },
        )
    })
    .collect();

  BarChart::default()
    .direction(Direction::Horizontal)
    .bar_gap(0)
    .max(3)
    .data(BarGroup::default().bars(&bars))
}

pub fn year_smoke_records_bar_chart(state: &State) -> BarChart<'_> {
  let today = Local::now().date_naive();
  let cur_month = Month::try_from(today.month0() as u8 + 1).unwrap();
  let past_year = iter_months(cur_month).skip(1).take(12);
  let bars: Vec<_> = past_year
    .map(|month| {
      let label = month.name()[0..3].to_string();
      let value = state.recs_by_month.get(&month).map_or(0, |v| *v);
      Bar::default().label(label.into()).value(value as _)
    })
    .collect();

  let max_val = state.recs_by_month.values().max().map_or(0, |v| *v);

  BarChart::default()
    .bar_width(3)
    .bar_gap(1)
    .max(bar_max(max_val as _))
    .data(BarGroup::default().bars(&bars))
}

fn iter_months(start: Month) -> impl Iterator<Item = Month> {
  (0..).scan(start, |next, _| {
    let cur = *next;
    *next = cur.succ();
    Some(cur)
  })
}

pub fn help_paragraph<'a>() -> Paragraph<'a> {
  Paragraph::new("SPACE - add record, u - undo, U - redo, ESC - quit")
}
