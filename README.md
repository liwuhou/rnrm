# rnrm

A Rust-powered tool designed to streamline the management of npm's registry.

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
        **Addition**
-   [ ] friendly fuzzy keyword
-   [ ] friendly prompt?
-   [ ] Scan the current npm project automatically, search for `.npmrc`, `.yarnrc`, `.pnpmrc` files, and read the registry configuration from them.
-   [ ] Check the correctness of a URL.
