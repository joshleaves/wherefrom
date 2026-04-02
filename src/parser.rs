use std::path::PathBuf;

/// Parsed command-line arguments for a normal `wherefrom` invocation.
#[derive(Debug, PartialEq, Eq)]
pub struct CliArgs {
  /// Whether all recorded origins should be printed.
  pub all: bool,
  /// Input files to inspect.
  pub files: Vec<PathBuf>,
}

/// High-level result of parsing the command line.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseOutcome {
  /// Continue with a normal run.
  Run(CliArgs),
  /// Print help text and exit successfully.
  Help,
  /// Print version text and exit successfully.
  Version,
}

/// Parses `wherefrom` command-line arguments.
///
/// Supports `-a`/`--all`, `-h`/`--help`, `-v`/`--version`, and `--` to stop
/// option parsing.
///
/// Returns an error string formatted for direct display on stderr when parsing
/// fails.
pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<ParseOutcome, String> {
  let mut all = false;
  let mut files = Vec::new();
  let mut parsing_options = true;

  for arg in args {
    if parsing_options {
      if arg == "--" {
        parsing_options = false;
        continue;
      }
      if arg == "--version" || arg == "-v" {
        return Ok(ParseOutcome::Version);
      }
      if arg == "--help" || arg == "-h" {
        return Ok(ParseOutcome::Help);
      }
      if arg == "--all" || arg == "-a" {
        all = true;
        continue;
      }
      if arg.starts_with('-') {
        return Err(format!("wherefrom: unrecognized option '{}'", arg));
      }
    }

    files.push(PathBuf::from(arg));
  }

  if files.is_empty() {
    return Err("wherefrom: missing file operand".to_string());
  }

  Ok(ParseOutcome::Run(CliArgs { all, files }))
}

#[cfg(test)]
mod tests {
  use super::{ParseOutcome, parse_args};
  use std::path::PathBuf;

  #[test]
  fn parses_all_and_files() {
    let result = parse_args([
      String::from("--all"),
      String::from("a.jpg"),
      String::from("b.png"),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(args.all);
    assert_eq!(
      args.files,
      vec![
        PathBuf::from("a.jpg"),
        PathBuf::from("b.png")
      ]
    );
  }

  #[test]
  fn parses_short_all_flag() {
    let result = parse_args([
      String::from("-a"),
      String::from("a.jpg"),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(args.all);
    assert_eq!(
      args.files,
      vec![PathBuf::from(
        "a.jpg"
      )]
    );
  }

  #[test]
  fn parses_help() {
    let result = parse_args([String::from(
      "--help",
    )])
    .unwrap();
    assert!(matches!(result, ParseOutcome::Help));
  }

  #[test]
  fn parses_version() {
    let result = parse_args([String::from(
      "--version",
    )])
    .unwrap();
    assert!(matches!(result, ParseOutcome::Version));
  }

  #[test]
  fn parses_short_version_flag() {
    let result = parse_args([String::from("-v")]).unwrap();
    assert!(matches!(result, ParseOutcome::Version));
  }

  #[test]
  fn stops_parsing_options_after_double_dash() {
    let result = parse_args([
      String::from("--"),
      String::from("--file.jpg"),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(!args.all);
    assert_eq!(
      args.files,
      vec![PathBuf::from(
        "--file.jpg"
      )]
    );
  }

  #[test]
  fn rejects_unknown_options() {
    let result = parse_args([String::from(
      "--wat",
    )])
    .unwrap_err();
    assert_eq!(result, "wherefrom: unrecognized option '--wat'");
  }

  #[test]
  fn requires_at_least_one_file() {
    let result = parse_args(Vec::<String>::new()).unwrap_err();
    assert_eq!(result, "wherefrom: missing file operand");
  }
}
