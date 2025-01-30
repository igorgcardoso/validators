all: build-node build-bundler

build-node:
    wasm-pack build --target nodejs --out-dir pkg-nodejs

build-bundler:
    wasm-pack build --target bundler --out-dir pkg-bundler
