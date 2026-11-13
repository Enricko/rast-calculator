# rast-calculator

A tiny dependency-free integer calculator, packaged for the RAST Package
Center with the [RAST package template](https://github.com/Enricko/rast-package-template).
It is written in Rust and built by `rustc` in CI; every release ships one static
archive per Linux target plus `rast-package.json`.

## Install in RAST

Package Center → **Install from GitHub** → paste
<https://github.com/Enricko/rast-calculator/releases/latest> (or a specific
release link), review the preview, approve the source and install. The
`calculator` command becomes available in the terminal / login shells.

```sh
calculator "2 + 3 * (4 - 1)"     # prints 11
calculator --version             # prints Calculator 1.0.0
```

## Development

```sh
rustc --edition 2021 -O src/main.rs -o /tmp/calculator
/tmp/calculator "2 + 3 * 4"
bash scripts/build.sh linux-x86_64 dist
```

Publishing: push a tag like `v1.0.1-1`; the release workflow builds both targets,
runs the Package Center checks and publishes the manifest and archives.
