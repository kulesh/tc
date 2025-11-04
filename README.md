# tc - Token Counter

A command-line utility for counting LLM tokens in files, similar to Unix `wc` (word count).

## Features

- Count tokens using Hugging Face tokenizers
- Multiple file support with totals
- Read from stdin or files
- Flexible output options (tokens, lines, bytes)
- Similar interface to Unix `wc`

## Installation

### Homebrew (macOS and Linux)

```bash
brew install kulesh/tap/tc
```

Or tap first, then install:

```bash
brew tap kulesh/tap
brew install tc
```

### From Source

```bash
cargo install --git https://github.com/kulesh/tc
```

Or build locally:

```bash
cargo build --release
# Binary will be at target/release/tc
```

## Usage

### Basic Usage

`tc` works just like `wc` - no configuration needed! It uses an embedded GPT-2 tokenizer by default.

Count tokens in a file:
```bash
tc file.txt
```

Count tokens in multiple files:
```bash
tc file1.txt file2.txt file3.txt
```

Read from stdin:
```bash
cat file.txt | tc
echo "Hello, world!" | tc
```

### Output Options

By default, `tc` shows tokens, lines, and bytes (similar to `wc`):
```bash
$ tc example.txt
     245       10     1024 example.txt
```

Show only token count:
```bash
tc --tokens-only file.txt
```

Show specific metrics:
```bash
tc --lines --bytes file.txt
```

### Custom Tokenizers

By default, `tc` uses an embedded GPT-2 tokenizer. To use a different tokenizer, provide a path with `--tokenizer`:

```bash
tc --tokenizer custom-tokenizer.json file.txt
```

You can download other tokenizers from Hugging Face:

**GPT-4 (cl100k_base):**
```bash
# Download from OpenAI's tiktoken repository or Hugging Face models
```

**BERT:**
```bash
curl -o bert-tokenizer.json https://huggingface.co/bert-base-uncased/resolve/main/tokenizer.json
tc --tokenizer bert-tokenizer.json file.txt
```

## Examples

Compare token counts across files:
```bash
tc *.md
```

Count tokens from command output:
```bash
git log --oneline | tc
```

Show only lines and tokens:
```bash
tc --lines file.txt
```

Check token count before sending to an API:
```bash
cat prompt.txt | tc
```

## Development

See [CLAUDE.md](CLAUDE.md) for development instructions.

Quick commands:
```bash
# Build
cargo build

# Run tests
cargo nextest run

# Run the binary (uses embedded GPT-2 tokenizer)
cargo run -- file.txt

# Format code
cargo fmt

# Lint
cargo clippy
```

## Releases

To create a new release:

1. **Update the version** in `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.1.2"  # Bump to new version
   ```

2. **Commit the version bump**:
   ```bash
   git add Cargo.toml
   git commit -m "Release v0.1.2"
   ```

3. **Create and push a git tag**:
   ```bash
   git tag v0.1.2
   git push origin main
   git push origin v0.1.2
   ```

4. **GitHub Actions handles the rest automatically**:
   - Builds binaries for multiple platforms (macOS, Linux, Windows)
   - Creates a GitHub Release with artifacts
   - Generates and publishes Homebrew formula to [`kulesh/homebrew-tap`](https://github.com/kulesh/homebrew-tap)

   Monitor the workflow at: https://github.com/kulesh/tc/actions

Once the workflow completes successfully, users can install the new version via:
```bash
brew upgrade kulesh/tap/tc
```

## Architecture

This is a Rust workspace with two crates:
- `token-counter-lib`: Core library for token counting
- `token-counter-bin`: CLI application (binary name: `tc`)

## License

MIT OR Apache-2.0
