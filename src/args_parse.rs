#[derive(Debug, PartialEq)]
pub enum Command {
  Add,
  Details(usize),
  Report(usize),
  Help,
}

pub fn parse(args: &[String]) -> Option<Command> {
  match args.first()?.to_lowercase().replace("-", "").as_str() {
    "add" | "a" => Some(Command::Add),
    "report" | "r" => {
      let n = args
        .iter()
        .nth(1)
        .and_then(|it| it.parse::<isize>().ok())
        .unwrap_or(0);
      if n > 0 {
        return Some(Command::Report(n as _));
      }
      Some(Command::Details(n.abs_diff(0)))
    }
    "help" | "h" => Some(Command::Help),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_add_command() {
    let cmd = parse(&["add".to_string()]);
    assert_eq!(cmd, Some(Command::Add));
  }

  #[test]
  fn parse_report_command() {
    let cmd = parse(&["Report".to_string()]);
    assert_eq!(cmd, Some(Command::Details(1)));
  }

  #[test]
  fn parse_help_command() {
    let cmd = parse(&["-h".to_string()]);
    assert_eq!(cmd, Some(Command::Help));
  }
}
