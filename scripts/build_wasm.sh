cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./hoops/ --target web ./target/wasm32-unknown-unknown/release/Hoops.wasm
