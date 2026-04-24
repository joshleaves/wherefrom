#![cfg(target_os = "macos")]

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

fn cmd_output(program: &str, args: &[&str]) -> String {
  match Command::new(program).args(args).output() {
    Ok(output) => {
      let mut text = String::new();
      text.push_str(&String::from_utf8_lossy(&output.stdout));
      if !output.stderr.is_empty() {
        text.push_str("\n[stderr]\n");
        text.push_str(&String::from_utf8_lossy(&output.stderr));
      }
      text
    }
    Err(e) => format!("failed to execute {program}: {e}"),
  }
}

fn set_wherefroms_with_osxmetadata(
  path: &Path,
  origin1: &str,
  origin2: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let output = Command::new("uvx")
    .args([
      "osxmetadata",
      "--set",
      "wherefroms",
      origin1,
      "--set",
      "wherefroms",
      origin2,
    ])
    .arg(path)
    .output()?;
  if !output.status.success() {
    return Err(
      format!(
        "uvx osxmetadata failed with status {} and stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
      )
      .into(),
    );
  }

  Ok(())
}

fn metadata_path_for(file: &Path) -> String {
  cmd_output(
    "/usr/bin/mdls",
    &[
      "-raw",
      "-name",
      "kMDItemPath",
      file.to_str().unwrap_or(""),
    ],
  )
  .trim()
  .to_string()
}

fn metadata_wherefroms_for(file: &Path) -> String {
  cmd_output(
    "/usr/bin/mdls",
    &[
      "-raw",
      "-name",
      "kMDItemWhereFroms",
      file.to_str().unwrap_or(""),
    ],
  )
  .trim()
  .to_string()
}

fn normalize_spotlight_path(path: &str) -> String {
  const DATA_PREFIX: &str = "/System/Volumes/Data";
  if let Some(stripped) = path.strip_prefix(DATA_PREFIX) {
    stripped.to_string()
  } else {
    path.to_string()
  }
}

#[ignore]
#[test]
fn e2e_reads_wherefroms_from_spotlight_metadata() -> Result<(), Box<dyn std::error::Error>> {
  let uvx_status = Command::new("uvx").arg("--version").output();
  match uvx_status {
    Ok(output) if output.status.success() => {}
    Ok(output) => {
      return Err(
        format!(
          "guard failed: `uvx --version` returned {} with stderr: {}",
          output.status,
          String::from_utf8_lossy(&output.stderr)
        )
        .into(),
      );
    }
    Err(e) => {
      return Err(format!("guard failed: `uvx` not found in PATH or not executable: {e}").into());
    }
  }

  let out_dir = Path::new(&std::env::var("HOME")?).join("Downloads");
  let file = out_dir.join(format!("wherefrom-it-{}.jpg", Uuid::new_v4()));
  File::create(&file)?;
  let file: PathBuf = file.canonicalize()?;

  let origin1 = format!("https://example.com/{}", Uuid::new_v4());
  let origin2 = format!("https://example.com/{}", Uuid::new_v4());
  let expected = vec![
    origin1.clone(),
    origin2.clone(),
  ];

  set_wherefroms_with_osxmetadata(&file, &origin1, &origin2)?;

  let exe = std::env::var("CARGO_BIN_EXE_wherefrom")?;
  let mut last_lines: Vec<String> = Vec::new();
  let file_str = file.to_string_lossy().to_string();
  let cwd = std::env::current_dir()?;
  let mut last_md_path = String::new();
  let mut last_md_wherefroms = String::new();

  for _ in 0..25 {
    last_md_path = metadata_path_for(&file);
    last_md_wherefroms = metadata_wherefroms_for(&file);
    let output = Command::new(&exe).args(["--all"]).arg(&file).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    last_lines = stdout.lines().map(|line| line.to_string()).collect();

    let md_path_matches = normalize_spotlight_path(&last_md_path) == file_str;
    if output.status.success() && md_path_matches && last_lines == expected {
      fs::remove_file(&file)?;
      return Ok(());
    }

    thread::sleep(Duration::from_millis(250));
  }

  let xattr = cmd_output("/usr/bin/xattr", &["-l", &file_str]);
  let _ = fs::remove_file(&file);
  Err(
    format!(
      "wherefrom output did not match expected origins. expected={expected:?}, got={last_lines:?}\n\
cwd={cwd:?}\n\
file={file_str}\n\
mdls.kMDItemPath={last_md_path:?}\n\
mdls.kMDItemWhereFroms={last_md_wherefroms:?}\n\
xattr:\n{xattr}"
    )
    .into(),
  )
}
