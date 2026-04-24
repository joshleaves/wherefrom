# wherefrom

Small macOS CLI to read a file’s origin URL(s) from Spotlight metadata (`kMDItemWhereFroms`).

## Features

- Print the primary origin (default)
- Print all origins with `--all`
- Handles multiple files
- Unix-friendly output (stdout for data, stderr for errors)

## Install

### Homebrew

```bash
brew tap joshleaves/tap
brew install wherefrom
```

### Manual

Download the appropriate binary from the [GitHub Releases page](https://github.com/joshleaves/wherefrom/releases/):

- `wherefrom-aarch64-apple-darwin` (Apple Silicon)
- `wherefrom-x86_64-apple-darwin` (Intel)

Then make it executable and move it to your `$PATH`:

```bash
chmod +x wherefrom-*
mv wherefrom-* /usr/local/bin/wherefrom
```

## Usage

```bash
wherefrom [--all] [--print0|--jsonl] file ...
```

### Examples

Print the primary origin:

```bash
wherefrom video.mp4
```

Print all origins:

```bash
wherefrom --all video.mp4
```

Show help:

```bash
wherefrom --help
```

Show version:

```bash
wherefrom --version
```

Multiple files:

```bash
wherefrom a.mp4 b.jpg
# a.mp4: https://...
# b.jpg: https://...
```

Machine-readable output (`NUL` delimited):

```bash
wherefrom --print0 a.mp4 b.jpg
```

Machine-readable output (JSON Lines):

```bash
wherefrom --jsonl a.mp4 b.jpg
# {"file":"a.mp4","origin":"https://..."}
# {"file":"b.jpg","origin":"https://..."}
```

## Output

- **Single file**: prints the first origin only
- **Multiple files**: `filename: origin`
- If no origin is found: no output for that file
- Errors are printed to stderr
- This default output is human-oriented, so unusual filenames can look ambiguous

## Options

- `-a`, `--all`: print all recorded origins
- `--print0`: print machine-readable NUL-delimited output
  - single file mode: `origin\0`
  - multi-file mode: repeated `path\0origin\0`
- `--jsonl`: print machine-readable JSON Lines output
  - one JSON object per origin: `{"file":"...","origin":"..."}`
- `-h`, `--help`: display help and exit
- `-v`, `--version`: display version and exit

## Notes

- macOS only (uses CoreFoundation / MDItem)
- `kMDItemWhereFroms` is a list; by default only the first value is printed

## Build

```bash
cargo build --release
```

## Local Hooks

This repository currently uses the following local Git hook:
- `.git/config`
```
[hook "lint"]
        event = pre-commit
        command = cargo fmt --check >/dev/null 2>&1
[hook "clippy"]
        event = pre-commit
        command = cargo clippy --all-targets -- -D warnings -D clippy::all >/dev/null 2>&1
```
- `.git/hooks/pre-push`

```bash
#!/bin/sh
set -eu

# Git pre-push args:
#   $1 = remote name
#   $2 = remote URL
# Stdin lines:
#   <local-ref> <local-oid> <remote-ref> <remote-oid>

run_checks=0

while IFS=' ' read -r local_ref local_oid remote_ref remote_oid; do
  [ -n "${local_ref:-}" ] || continue

  case "$local_ref" in
    refs/tags/v*)
      # Ignore tag deletion (all-zero local oid)
      case "$local_oid" in
        0000000000000000000000000000000000000000) ;;
        *) run_checks=1 ;;
      esac
      ;;
  esac
done

if [ "$run_checks" -eq 1 ]; then
  cargo test --test integration_spotlight -- --ignored
fi

exit 0
```

## License

MIT
