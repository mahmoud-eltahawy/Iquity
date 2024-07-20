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
1. [Rust](https://www.rust-lang.org/)
2. [Tauri V2](https://v2.tauri.app/start/prerequisites/)
3. [Leptos](https://leptos.dev/)
```bash
  cargo install trunk
  rustup target add wasm32-unknown-unknown
```
4. iquity is using [Bun](https://bun.sh/) to download tailwind and daisyui

### Installing
```
  git clone --depth=1 https://github.com/mahmoud-eltahawy/Iquity.git
  cd iquity
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
### the help message
Welcom to iquity 
    the markdown compiler

you called iquity without a markdown path

you should call the program with the path to
the target md file then the program will hot reload
the content of the file every time you change 
something in it. 


EXAMPLE

iquity ./README.md


PREVIEW WINDOW KEYS    

p => print to pdf

j => next theme

k => previous theme

= or + => increase font size    

- or _ => decrease font size    

? or / => show this help message    

esc => to hide this message    



## how is this different from Ubiquity
- Ubiquity ships it's own text editor side by side with the previewer while iquity trys to be minimal by watching the file you are editing with your favourite text editor and hot reloads it's content every time you change some thing in it
- Ubiqity is using **Yew** while iquity using **Leptos**
- some other minimal differencs you will notice while using it but as development goes on. the two projects will become very different

## todos
- [ ] add a toml config file
- [ ] add slides feature


