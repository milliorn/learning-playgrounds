# Create a Rust worker

<https://workers.wasmlabs.dev/docs/tutorials/rust-workers/>

## In this case, you need to compile the project to Wasm (WASI)

### Install the component and build

`rustup target add wasm32-wasi && cargo build --release --target wasm32-wasi`

## Install the wws WWS server

<https://workers.wasmlabs.dev/docs/start/>

## Run WWS server

`wws .`

## Finally, open <http://127.0.0.1:8080/worker> in your browser

 <http://127.0.0.1:8080/worker>
