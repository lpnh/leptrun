# 🦀 leptrun

A CSR website template leveraging
[Leptos](https://github.com/leptos-rs/leptos),
[Trunk](https://github.com/trunk-rs/trunk) and
[Tailwind](https://github.com/tailwindlabs/tailwindcss).

## Setup

### Compiling to WebAssembly

Install the Wasm target:

```
rustup target add wasm32-unknown-unknown
```

To check the installed targets:

```
rustup target list --installed
```

### Trunk

Install Trunk using cargo:

```
cargo install trunk --locked
```

*For additional installation options, refer to the [install
section](https://trunkrs.dev/#install) on Trunk's website*

### Tailwind (standalone CLI)

Get the [latest
release](https://github.com/tailwindlabs/tailwindcss/releases/latest) and give
it executable permissions — no Node.js or npm required.

Example for Linux:

```
curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
```

### Task Runner

Install cargo-make:

```
cargo install --force cargo-make
```

*For additional installation options, refer to the [installation
section](https://github.com/sagiegurari/cargo-make?tab=readme-ov-file#installation)
on cargo-make repo*

## Usage

### Running it

Simply leverage the tasks available in the `Makefile.toml`.

**Development**

```no_rust
cargo make run-dev
```

**Release**

```no_rust
cargo make run-release
```

**Updating `gh-pages` branch**

```
cargo make update-pages
```

### Renaming it

Make sure to update the following files to match your project name.

`Cargo.toml`

```toml
[package]
name = "leptrun"
```

`Makefile.toml`

```toml
[tasks.build-release]
command = "trunk"
args = ["build", "--config", "Release.toml", "--public-url", "/leptrun/"]
```

`index.html`

```html
<title>leptrun</title>
```

## Template Walkthrough

### Tailwind Configuration

To generate the `tailwind.config.js` config file:

```
./tailwindcss init
```

Update it to include html and Rust files:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}

```

Add the `@tailwind` directives to the `input.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Including the `Fira Mono` font:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
    fontFamily: {
      'fira-mono': ['"Fira Mono"', 'monospace'],
    },
  },
  plugins: [],
}

```

```css
@import url('https://fonts.googleapis.com/css2?family=Fira+Mono&display=swap');
@tailwind base;
@tailwind components;
@tailwind utilities;
```
