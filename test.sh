#!/bin/bash

cargo test --all --features "with-titik with-web with-gtk"


# Install wasm-pack if it isn't installed yet
if ! type wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

wasm-pack test --firefox --headless
