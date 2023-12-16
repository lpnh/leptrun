# leptrun 🦀

My personal CSR Web App template leveraging Leptos, Trunk, and TailwindCSS

## Setup

### Compiling to WebAssembly

To be able to compile the code for WebAssembly, run:

```no_rust
rustup target add wasm32-unknown-unknown
```

### Trunk

Install [Trunk](https://github.com/trunk-rs/trunk) using cargo:

```no_rust
cargo install trunk
```

### Tailwind (standalone CLI)

Get the [latest release](https://github.com/tailwindlabs/tailwindcss/releases/latest) and give it executable permissions:

```no_rust
curl -LO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
```

### Assets

Create new `public` directory for storing `favicon.ico`

```no_rust
mkdir public
```

### Makefile

Install `cargo-make`:

```no_rust
sudo pacman -S cargo-make
```

## VS Code

Add the following to `settings.json` file:

```json
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false,
```

## Usage

### Development

```no_rust
cargo make run-dev
```

### Release

```no_rust
cargo make run-release
```

### Updating `gh-pages` branch

```
cargo make update-pages
```