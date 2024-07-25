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

## how to install it
at the moment. building from source is the only option

### Prerequisites
1. install [Rust](https://www.rust-lang.org/)
2. make sure your machine have [Tauri V2](https://v2.tauri.app/start/prerequisites/) Prerequisites
4. install [Bun](https://bun.sh/) to download tailwind and daisyui

### Installing
```bash
  git clone --depth=1 https://github.com/mahmoud-eltahawy/Iquity.git
  cd iquity
  cargo install trunk
  rustup target add wasm32-unknown-unknown
  cargo install tauri-cli --version "^2.0.0-beta"
  cargo tauri build
```
- the builded program is called iquity located at ./target/release/iquity
- it is single executable so you just move it to a directory that is in your path

## how to use it
```bash
  iquity # will print help messge
  # you should use it like this
  iquity PATH_TO_README_FILE.md
```

## how is this different from Ubiquity
- Ubiquity ships it's own text editor side by side with the previewer while iquity trys to be minimal by watching the file you are editing with your favourite text editor and hot reloads it's content every time you change some thing in it
- Ubiqity is using **Yew** while iquity using **Leptos**
- some other minimal differencs you will notice while using it but as development goes on. the two projects will become very different

## todos
- [X] **ADD**: slides feature
- [X] **ADD**: a toml config file
- [X] **ADD**: keybindings to config
- [ ] **FIX BUG** : some editors remove and retouch the file it edits and notify crate has surprising behavior for that on some platforms
