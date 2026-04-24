use mdquery_rs::MDItem;
use objc2_core_foundation::CFArray;
use std::io::{self, ErrorKind};
use std::process::ExitCode;
use wherefrom::{
  origins::wherefrom_origins,
  parser::{ParseOutcome, parse_args},
  printer::{OutputScope, OutputStrategy, origin_limit},
};

fn usage() -> &'static str {
  "\
usage: wherefrom [--all] [--print0|--jsonl] file ...

Display file origin URL(s) from Spotlight metadata.

Options:
  -a, --all     display all origins
      --print0  print machine-readable NUL-delimited records
      --jsonl   print machine-readable JSON Lines
  -h, --help    display this help and exit
  -v, --version display version"
}

fn version() -> String {
  let digest = "wherefrom: Display file origin URL(s) from Spotlight metadata.";
  let version = env!("CARGO_PKG_VERSION");
  let git = env!("CARGO_PKG_REPOSITORY");
  format!("{}\nv{}\n{}", digest, version, git)
}

fn main() -> ExitCode {
  let args = match parse_args(std::env::args().skip(1)) {
    Ok(ParseOutcome::Help) => {
      println!("{}", usage());
      return ExitCode::SUCCESS;
    }
    Ok(ParseOutcome::Version) => {
      println!("{}", version());
      return ExitCode::SUCCESS;
    }
    Ok(ParseOutcome::Run(args)) => args,
    Err(message) => {
      eprintln!("{}", message);
      eprintln!("{}", usage());
      return ExitCode::FAILURE;
    }
  };

  let mut had_error: bool = false;
  let scope = if args.files.len() == 1 {
    OutputScope::SingleFile
  } else {
    OutputScope::MultiFile
  };
  let strategy = if args.print0 {
    OutputStrategy::Print0(scope)
  } else if args.jsonl {
    OutputStrategy::JsonL
  } else {
    OutputStrategy::Human(scope)
  };
  let mut stdout = io::stdout().lock();

  for file in &args.files {
    let md = match MDItem::from_path(file.as_path()) {
      Ok(md) => md,
      Err(e) => {
        eprintln!("wherefrom: {}: {}", file.display(), e);
        had_error = true;
        continue;
      }
    };
    let origins = match md.get_attribute::<CFArray>("kMDItemWhereFroms") {
      None => continue,
      Some(origins) => origins,
    };

    let origins = wherefrom_origins(&origins);
    let limit = origin_limit(origins.len() as isize, args.all) as usize;

    for origin in origins.into_iter().take(limit) {
      if let Err(e) = strategy.write(&mut stdout, file, &origin) {
        if e.kind() == ErrorKind::BrokenPipe {
          return ExitCode::SUCCESS;
        }
        eprintln!("wherefrom: failed to write output: {}", e);
        had_error = true;
        break;
      }
    }
  }
  if had_error {
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
