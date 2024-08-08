# Iquity
A cross-platform markdown compiler.

## Why should I use it?
- ✨ Over 30 themes. ✨
- 🦋 Beautiful UI - Tailwind and DaisyUI. 🦋
- 📂 PDF Export - Printable markdown preview. 📂
- 🦀 Rusty - Built with Rust, frontend and backend. 🦀

## How is it made?
- [Leptos](https://www.leptos.dev) - A web application framework.
- [Tauri](https://www.tauri.app) - A desktop application framework.
- [Tailwind](https://www.tailwindcss.com) - A CSS framework.
- [DaisyUI](https://www.daisyui.com) - A Tailwind CSS component library.

---

## how to install it
at the moment. building from source is the only option

### Prerequisites
1. install [Rust](https://www.rust-lang.org/)
2. make sure your machine have [Tauri V2](https://v2.tauri.app/start/prerequisites/) Prerequisites

### Installing
```sh
  git clone --depth=1 https://github.com/mahmoud-eltahawy/Iquity.git
  cd iquity
  cargo install trunk
  rustup target add wasm32-unknown-unknown
  cargo install tauri-cli --version "^2.0.0-beta"
  cargo tauri build
```
1. the builded program is called iquity located at ./target/release/iquity
1. you just move it to a directory that is in your path

---

## how to use it
```sh
  iquity # will print help messge
  # you should use it like this
  iquity PATH_TO_README_FILE.md
```
