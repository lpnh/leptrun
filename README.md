# 🦀 leptrun

A CSR website template leveraging
[Leptos](https://github.com/leptos-rs/leptos),
[Trunk](https://github.com/trunk-rs/trunk) and
[Tailwind](https://github.com/tailwindlabs/tailwindcss)

## Generating the Template

This template can be generated using `cargo-generate`:

```
cargo generate git@github.com:lpnh/leptrun.git
```

*For additional information, check its [documentation](https://cargo-generate.github.io/cargo-generate/index.html)*

## Setup

### Compiling to WebAssembly

Install the Wasm target:

```sh
rustup target add wasm32-unknown-unknown
```

To check the installed targets:

```sh
rustup target list --installed
```

### Trunk

Install Trunk:

```sh
cargo install trunk --locked
```

*For additional installation options, refer to the [install
section](https://trunkrs.dev/#install) on Trunk's website*

### Cargo-make

Install cargo-make:

```sh
cargo install --force cargo-make
```

*For additional installation options, refer to the [installation
section](https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation)
on cargo-make repo*

## Usage

Feel free to leverage the tasks available in the `Makefile.toml`.

**Development**

```sh
cargo make run-dev
```

**Release**

```sh
cargo make run-release
```

**Updating `gh-pages` branch**

```sh
cargo make update-pages
```
