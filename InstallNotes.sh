cd $srcdir/${pkgname}-${pkgver}
rustup update nightly-2023-07-07-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-2023-07-07-x86_64-unknown-linux-gnu
cargo install trunk
cargo install tauri-cli
rustup target add wasm32-unknown-unknown
cd frontend
npm install
npx tailwindcss -i ./css/input.css -o ./css/output.css --minify
cd ..
cargo tauri build -b none --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
