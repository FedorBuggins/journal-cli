#[derive(Debug, PartialEq)]
pub enum Command {
  Add,
  Report,
  Help,
}

pub fn parse(args: &[String]) -> Option<Command> {
  match args.first()?.to_lowercase().replace("-", "").as_str() {
    "add" | "a" => Some(Command::Add),
    "report" | "r" => Some(Command::Report),
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
    assert_eq!(cmd, Some(Command::Report));
  }

  #[test]
  fn parse_help_command() {
    let cmd = parse(&["-h".to_string()]);
    assert_eq!(cmd, Some(Command::Help));
  }
}
