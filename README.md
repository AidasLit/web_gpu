setup dev enviroment (linux):

1. (WSL)(debian-based) `sudo apt install build-essential`
2. install rustup
3. `rustup target add wasm32-unknown-unknown`
4. `cargo install wasm-pack`
5. (debian-based) `sudo apt install python3 python3-pip ipython3`

build the app for WebGPU:
`wasm-pack build --target web`

start a local server to host the app:
`python -m http.server` or `python3 -m http.server`
