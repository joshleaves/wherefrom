use std::path::PathBuf;

/// Parsed command-line arguments for a normal `wherefrom` invocation.
#[derive(Debug, PartialEq, Eq)]
pub struct CliArgs {
  /// Whether all recorded origins should be printed.
  pub all: bool,
  /// Whether output should be NUL-delimited for machine parsing.
  pub print0: bool,
  /// Whether output should be JSON Lines for machine parsing.
  pub jsonl: bool,
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
/// Supports `-a`/`--all`, `--print0`, `--jsonl`, `-h`/`--help`,
/// `-v`/`--version`, and `--` to stop option parsing.
///
/// Returns an error string formatted for direct display on stderr when parsing
/// fails.
pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<ParseOutcome, String> {
  let mut all = false;
  let mut print0 = false;
  let mut jsonl = false;
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
      if arg == "--print0" {
        print0 = true;
        continue;
      }
      if arg == "--jsonl" {
        jsonl = true;
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
  if print0 && jsonl {
    return Err("wherefrom: options '--print0' and '--jsonl' are mutually exclusive".to_string());
  }

  Ok(ParseOutcome::Run(CliArgs {
    all,
    print0,
    jsonl,
    files,
  }))
}

#[cfg(test)]
mod tests {
  use super::{ParseOutcome, parse_args};
  use std::path::PathBuf;
  use uuid::Uuid;

  #[test]
  fn parses_all_and_files() {
    let file_a = format!("{}.jpg", Uuid::new_v4());
    let file_b = format!("{}.png", Uuid::new_v4());
    let result = parse_args([
      String::from("--all"),
      file_a.clone(),
      file_b.clone(),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(args.all);
    assert!(!args.print0);
    assert!(!args.jsonl);
    assert_eq!(
      args.files,
      vec![
        PathBuf::from(file_a),
        PathBuf::from(file_b)
      ]
    );
  }

  #[test]
  fn parses_short_all_flag() {
    let file = format!("{}.jpg", Uuid::new_v4());
    let result = parse_args([
      String::from("-a"),
      file.clone(),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(args.all);
    assert!(!args.print0);
    assert!(!args.jsonl);
    assert_eq!(args.files, vec![PathBuf::from(file)]);
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
    let value = format!("--{}.jpg", Uuid::new_v4());
    let result = parse_args([
      String::from("--"),
      value.clone(),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(!args.all);
    assert!(!args.print0);
    assert!(!args.jsonl);
    assert_eq!(args.files, vec![PathBuf::from(value)]);
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

  #[test]
  fn parses_print0() {
    let file = format!("{}.jpg", Uuid::new_v4());
    let result = parse_args([
      String::from("--print0"),
      file.clone(),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(!args.all);
    assert!(args.print0);
    assert!(!args.jsonl);
    assert_eq!(args.files, vec![PathBuf::from(file)]);
  }

  #[test]
  fn parses_jsonl() {
    let file = format!("{}.jpg", Uuid::new_v4());
    let result = parse_args([
      String::from("--jsonl"),
      file.clone(),
    ])
    .unwrap();

    let ParseOutcome::Run(args) = result else {
      panic!("expected run outcome");
    };

    assert!(!args.all);
    assert!(!args.print0);
    assert!(args.jsonl);
    assert_eq!(args.files, vec![PathBuf::from(file)]);
  }

  #[test]
  fn rejects_conflicting_machine_formats() {
    let result = parse_args([
      String::from("--print0"),
      String::from("--jsonl"),
      format!("{}.jpg", Uuid::new_v4()),
    ])
    .unwrap_err();
    assert_eq!(
      result,
      "wherefrom: options '--print0' and '--jsonl' are mutually exclusive"
    );
  }
}
