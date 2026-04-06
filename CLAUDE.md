# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build          # Build
cargo run            # Run (use `cargo run -- <command>` for subcommands)
cargo run -- use npm # Example: switch to npm registry
```

## Code Structure

- `src/main.rs` - Entry point, delegates to `Cli::run()`
- `src/lib.rs` - Core CLI implementation with `clap` derive parsers; subcommand handlers (`ls`, `current`, `use`, `add`, `set`, `del`, `rename`)
- `src/util.rs` - Registry operations: read/write `.npmrc` (global npm config) and `.nrmrc` (custom registries), switch registries, validate duplicates
- `src/util/error.rs` - `CustomError` type for user-facing errors
- `src/util/registries.rs` - Built-in registry definitions (npm, yarn, tencent, cnpm, taobao, npmMirror)

## Config Files

- `~/.npmrc` - npm's config file; stores current `registry=` setting
- `~/.nrmrc` - Custom registry definitions in INI format:
  ```
  [name]
  registry=https://...
  ```

## Notable Patterns

- Uses `OnceLock` for lazy initialization of registry maps
- Internal registries cannot be deleted/modified; operations return `CustomError`
- Error handling: `del` and `rename` downcast errors to `CustomError` for display; some paths have `FIXME` comments for incomplete error handling
