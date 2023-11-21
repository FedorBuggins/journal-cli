#[derive(Debug, PartialEq)]
pub enum Command {
  Add,
  Details(usize),
  Report(usize),
  Help,
}

pub fn parse(args: &[String]) -> Option<Command> {
  let mut args = args.iter();
  let cmd = args.next()?.to_lowercase().replace('-', "");
  let days_offset =
    args.next().and_then(|v| v.parse().ok()).unwrap_or(0);
  match cmd.as_str() {
    "add" | "a" => Some(Command::Add),
    "details" | "d" => Some(Command::Details(days_offset)),
    "report" | "r" => Some(Command::Report(days_offset)),
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
    assert_eq!(cmd, Some(Command::Report(0)));
  }

  #[test]
  fn parse_help_command() {
    let cmd = parse(&["-h".to_string()]);
    assert_eq!(cmd, Some(Command::Help));
  }
}
