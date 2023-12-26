# 🦀 leptrun

My personal CSR Website template leveraging [Leptos](https://github.com/leptos-rs/leptos), [Trunk](https://github.com/trunk-rs/trunk) and [Tailwind](https://github.com/tailwindlabs/tailwindcss).

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

The config file can be generated running:

```no_rust
./tailwindcss init
```

And make sure to include the rust files, like this:

```js
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
```

### Assets

Create new `public` directory for storing `favicon.ico`

```no_rust
mkdir public
```

*Note: to store any additional public files, besides the ones
included in the `index.html` head,
you can use the `public/aux` directory.*

### Makefile

Install `cargo-make`:

```no_rust
sudo pacman -S cargo-make
```

### VS Code

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

## Project Name

Make sure to update all files to match the new project name. This includes:

**`Cargo.toml` package name**

```toml
[package]
name = "leptrun"
```

**`Makefile.toml` build-release task**

```toml
[tasks.build-release]
command = "trunk"
args = ["build", "--config", "Release.toml", "--public-url", "/leptrun/"]
```

**`index.html` title element**

```html
<title>leptrun</title>
```

## Usage

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
