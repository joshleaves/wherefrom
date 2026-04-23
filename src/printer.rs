use objc2_core_foundation::CFIndex;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Returns the number of origins to print for a file.
pub fn origin_limit(total: isize, all: bool) -> CFIndex {
  if all { total } else { total.min(1) }
}

/// Output scope for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputScope {
  SingleFile,
  MultiFile,
}

/// Output format strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStrategy {
  Human(OutputScope),
  Print0(OutputScope),
  JsonL,
}

impl OutputStrategy {
  /// Writes one origin record to the provided writer.
  pub fn write<W: Write>(&self, out: &mut W, file: &Path, origin: &str) -> io::Result<()> {
    match self {
      Self::Human(scope) => writeln!(out, "{}", format_result(*scope, file, origin)),
      Self::Print0(scope) => out.write_all(&format_result_print0(*scope, file, origin)),
      Self::JsonL => writeln!(
        out,
        "{}",
        serde_json::json!({
          "file": file.to_string_lossy(),
          "origin": origin,
        })
      ),
    }
  }
}

/// Formats one output line for a resolved origin.
///
/// For single-file invocations, only the origin is returned. For multi-file
/// invocations, the result is prefixed with `path: `.
pub fn format_result(scope: OutputScope, file: &Path, origin: &str) -> String {
  match scope {
    OutputScope::SingleFile => origin.to_string(),
    OutputScope::MultiFile => format!("{}: {}", file.display(), origin),
  }
}

/// Formats one output record for `--print0`.
///
/// Single-file mode emits `origin\0`.
/// Multi-file mode emits `path\0origin\0`.
pub fn format_result_print0(scope: OutputScope, file: &Path, origin: &str) -> Vec<u8> {
  match scope {
    OutputScope::SingleFile => {
      let mut out = Vec::with_capacity(origin.len() + 1);
      out.extend_from_slice(origin.as_bytes());
      out.push(0);
      out
    }
    OutputScope::MultiFile => {
      let path = file.as_os_str().as_bytes();
      let mut out = Vec::with_capacity(path.len() + origin.len() + 2);
      out.extend_from_slice(path);
      out.push(0);
      out.extend_from_slice(origin.as_bytes());
      out.push(0);
      out
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{OutputScope, OutputStrategy, format_result, format_result_print0, origin_limit};
  use std::path::Path;
  use uuid::Uuid;

  #[test]
  fn returns_zero_when_no_origins_are_available() {
    assert_eq!(origin_limit(0, false), 0);
  }

  #[test]
  fn returns_one_when_not_showing_all_and_origins_exist() {
    assert_eq!(origin_limit(1, false), 1);
    assert_eq!(origin_limit(3, false), 1);
  }

  #[test]
  fn returns_all_origins_when_requested() {
    assert_eq!(origin_limit(3, true), 3);
  }

  #[test]
  fn formats_single_file_output_without_prefix() {
    let origin = format!("https://example.com/{}", Uuid::new_v4());
    assert_eq!(
      format_result(OutputScope::SingleFile, Path::new("video.mp4"), &origin),
      origin
    );
  }

  #[test]
  fn formats_multi_file_output_with_path_prefix() {
    let filename = format!("{}.mp4", Uuid::new_v4());
    let origin = format!("https://example.com/{}", Uuid::new_v4());
    assert_eq!(
      format_result(OutputScope::MultiFile, Path::new(&filename), &origin),
      format!("{}: {}", filename, origin)
    );
  }

  #[test]
  fn formats_print0_single_file_as_nul_terminated_origin() {
    let origin = format!("https://example.com/{}", Uuid::new_v4());
    let mut expected = origin.as_bytes().to_vec();
    expected.push(0);
    assert_eq!(
      format_result_print0(OutputScope::SingleFile, Path::new("ignored.mp4"), &origin),
      expected
    );
  }

  #[test]
  fn formats_print0_multi_file_as_path_nul_origin_nul() {
    let filename = format!("{}:\n{}.mp4", Uuid::new_v4(), Uuid::new_v4());
    let origin = format!("https://example.com/{}:{}", Uuid::new_v4(), Uuid::new_v4());
    let mut expected = filename.as_bytes().to_vec();
    expected.push(0);
    expected.extend_from_slice(origin.as_bytes());
    expected.push(0);
    assert_eq!(
      format_result_print0(OutputScope::MultiFile, Path::new(&filename), &origin),
      expected
    );
  }

  #[test]
  fn writes_jsonl_output() {
    let filename = format!("{}.mp4", Uuid::new_v4());
    let origin = format!("https://example.com/{}", Uuid::new_v4());
    let mut out = Vec::new();
    OutputStrategy::JsonL
      .write(&mut out, Path::new(&filename), &origin)
      .unwrap();

    let value: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(value["file"], filename);
    assert_eq!(value["origin"], origin);
  }
}
