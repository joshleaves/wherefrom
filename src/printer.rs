use objc2_core_foundation::CFIndex;
use std::path::Path;

/// Returns the number of origins to print for a file.
pub fn origin_limit(total: isize, all: bool) -> CFIndex {
  if all { total } else { total.min(1) }
}

/// Formats one output line for a resolved origin.
///
/// For single-file invocations, only the origin is returned. For multi-file
/// invocations, the result is prefixed with `path: `.
pub fn format_result(single_file: bool, file: &Path, origin: &str) -> String {
  if single_file {
    origin.to_string()
  } else {
    format!("{}: {}", file.display(), origin)
  }
}

#[cfg(test)]
mod tests {
  use super::{format_result, origin_limit};
  use std::path::Path;

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
    assert_eq!(
      format_result(true, Path::new("video.mp4"), "https://example.com/video"),
      "https://example.com/video"
    );
  }

  #[test]
  fn formats_multi_file_output_with_path_prefix() {
    assert_eq!(
      format_result(false, Path::new("video.mp4"), "https://example.com/video"),
      "video.mp4: https://example.com/video"
    );
  }
}
