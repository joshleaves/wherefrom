use argh::FromArgs;
use mdquery_rs::MDItem;
use objc2_core_foundation::{CFArray, CFString};
use std::path::{Path, PathBuf};

/// Display file origin using Apple's MDItem
#[derive(FromArgs)]
struct Args {
  /// display all origins
  #[argh(switch)]
  all: bool,

  /// input files
  #[argh(positional, greedy)]
  files: Vec<PathBuf>,
}

fn main() {
  let args: Args = argh::from_env();
  if args.files.is_empty() {
    eprintln!("wherefrom: no files");
    std::process::exit(1);
  }

  let mut had_error: bool = false;
  let single_file = args.files.len() == 1;

  for file in args.files {
    let md = match MDItem::from_path(&file) {
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

    let total = CFArray::count(&origins);
    let limit = if args.all { total } else { total.min(1) };

    for i in 0..limit {
      let value = unsafe { origins.value_at_index(i) };
      if value.is_null() {
        continue;
      }
      let origin = unsafe { (&*(value as *const CFString)).to_string() };
      print_result(single_file, &file, &origin);
    }
  }
  if had_error {
    std::process::exit(1);
  }
}

fn print_result(single_file: bool, file: &Path, origin: &str) {
  if single_file {
    println!("{}", origin);
  } else {
    println!("{}: {}", file.display(), origin);
  }
}
