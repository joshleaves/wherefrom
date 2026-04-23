use objc2_core_foundation::CFIndex;
use std::os::unix::ffi::OsStrExt;
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

/// Formats one output record for `--print0`.
///
/// Single-file mode emits `origin\0`.
/// Multi-file mode emits `path\0origin\0`.
pub fn format_result_print0(single_file: bool, file: &Path, origin: &str) -> Vec<u8> {
  if single_file {
    let mut out = Vec::with_capacity(origin.len() + 1);
    out.extend_from_slice(origin.as_bytes());
    out.push(0);
    out
  } else {
    let path = file.as_os_str().as_bytes();
    let mut out = Vec::with_capacity(path.len() + origin.len() + 2);
    out.extend_from_slice(path);
    out.push(0);
    out.extend_from_slice(origin.as_bytes());
    out.push(0);
    out
  }
}

#[cfg(test)]
mod tests {
  use super::{format_result, format_result_print0, origin_limit};
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

  #[test]
  fn formats_print0_single_file_as_nul_terminated_origin() {
    assert_eq!(
      format_result_print0(true, Path::new("ignored.mp4"), "https://example.com/video"),
      b"https://example.com/video\0"
    );
  }

  #[test]
  fn formats_print0_multi_file_as_path_nul_origin_nul() {
    assert_eq!(
      format_result_print0(
        false,
        Path::new("video:\nclip.mp4"),
        "https://example.com/a:b"
      ),
      b"video:\nclip.mp4\0https://example.com/a:b\0"
    );
  }
}
