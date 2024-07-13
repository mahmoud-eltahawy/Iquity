
cd $srcdir/${pkgname}-${pkgver}
rustup update stable
rustup component add rust-src --toolchain stable
cargo install trunk
cargo install tauri-cli
rustup target add wasm32-unknown-unknown
cd frontend
bun install
bunx tailwindcss -i ./css/input.css -o ./css/output.css --minify
cd ..
cargo tauri build -b none --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
