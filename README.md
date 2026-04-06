# rnrm

A Rust-powered tool designed to streamline the management of npm's registry.

### Installation

**From GitHub Releases:**

Download the latest release for your platform from the [Releases page](https://github.com/rnrm/rnrm/releases).

**From source:**

```bash
git clone https://github.com/rnrm/rnrm.git
cd rnrm
cargo install --path .
```

### Usage

```plain
rnrm <COMMAND>

Commands:
  ls       List all the registries
  current  Show current registry
  use      Change current registry
  add      Add a custom registry
  set      Add or modify a custom registry
  del      Delete a custom registry
  rename   Change custom registry's name
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

### TODO

-   [x] help
-   [x] ls
-   [x] current
-   [x] use
-   [x] add
-   [ ] set
-   [x] del
-   [x] rename
-   [ ] handle errors

### Addition

-   [ ] friendly fuzzy keyword
-   [ ] friendly prompt?
-   [ ] Scan the current npm project automatically, search for `.npmrc`, `.yarnrc`, `.pnpmrc` files, and read the registry configuration from them.
-   [ ] Check the correctness of a URL.

### Build

**Local build:**

```bash
cargo build --release      # Build for current platform
./build.sh                 # Build for all platforms (requires cross-compilation setup)
./build.sh x86_64-apple-darwin  # Build for specific platform
```

**Supported targets:**
- `x86_64-unknown-linux-gnu` - Linux (x86_64)
- `x86_64-unknown-linux-musl` - Linux static binary (x86_64)
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon
- `x86_64-pc-windows-gnu` - Windows (x86_64)

**GitHub Actions:**

Pushing a tag like `v1.0.0` will automatically build all platforms and create a GitHub Release with binaries attached.
