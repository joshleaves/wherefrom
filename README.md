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
wherefrom <file> [file...]
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

Multiple files:

```bash
wherefrom a.mp4 b.jpg
# a.mp4: https://...
# b.jpg: https://...
```

## Output

- **Single file**: prints the first origin only
- **Multiple files**: `filename: origin`
- If no origin is found: no output for that file
- Errors are printed to stderr

## Notes

- macOS only (uses CoreFoundation / MDItem)
- `kMDItemWhereFroms` is a list; by default only the first value is printed

## Build

```bash
cargo build --release
```

## License

MIT
