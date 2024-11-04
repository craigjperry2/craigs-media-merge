# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Running The Project

Install dependencies:

```bash
cargo install tauri-cli --version '^2.0.0' --locked
cargo install trunk --locked
rustup target add wasm32-unknown-unknown
```

Then run the development server:

```bash
$ cargo tauri dev
```
