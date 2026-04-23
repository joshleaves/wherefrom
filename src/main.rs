use mdquery_rs::MDItem;
use objc2_core_foundation::CFArray;
use wherefrom::{
  origins::wherefrom_origins,
  parser::{ParseOutcome, parse_args},
  printer::{format_result, origin_limit},
};

fn usage(exit_code: i32) -> ! {
  let output = "\
usage: wherefrom [--all] file ...

Display file origin URL(s) from Spotlight metadata.

Options:
  -a, --all     display all origins
  -h, --help    display this help and exit
  -v, --version display version";

  if exit_code == 0 {
    println!("{}", output);
  } else {
    eprintln!("{}", output);
  }
  std::process::exit(exit_code);
}

fn version() -> ! {
  let digest = "wherefrom: Display file origin URL(s) from Spotlight metadata.";
  let version = env!("CARGO_PKG_VERSION");
  let git = env!("CARGO_PKG_REPOSITORY");
  println!("{}\nv{}\n{}", digest, version, git);
  std::process::exit(0);
}

fn main() {
  let args = match parse_args(std::env::args().skip(1)) {
    Ok(ParseOutcome::Help) => usage(0),
    Ok(ParseOutcome::Version) => version(),
    Ok(ParseOutcome::Run(args)) => args,
    Err(message) => {
      eprintln!("{}", message);
      usage(1);
    }
  };

  let mut had_error: bool = false;
  let single_file = args.files.len() == 1;

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
      println!("{}", format_result(single_file, file, &origin));
    }
  }
  if had_error {
    std::process::exit(1);
  }
}
