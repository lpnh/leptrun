# taileptrun 🦀

A CSR Web App template using TailwindCSS, Leptos and Trunk

## Setup

### Compiling to WebAssembly

To be able to compile the code for WebAssembly:

```no_rust
rustup target add wasm32-unknown-unknown
```

### Trunk

Install [Trunk](https://github.com/trunk-rs/trunk) using cargo:

```no_rust
cargo install trunk
```

### Tailwind (standalone CLI)

Get the [latest release](https://github.com/tailwindlabs/tailwindcss/releases/latest) and give it executable permissions.

Example for Linux:

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

## Run

In development mode:

```no_rust
cargo make run-dev
```

For release:

```no_rust
cargo make run-release
```
