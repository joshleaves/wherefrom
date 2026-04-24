//! Library helpers for parsing `wherefrom` CLI arguments and formatting output.
#[cfg(not(target_os = "macos"))]
compile_error!("only macos is supported");

/// Safe extraction of `kMDItemWhereFroms` origin strings.
pub mod origins;
/// Command-line argument parsing.
pub mod parser;
/// Output formatting helpers.
pub mod printer;
