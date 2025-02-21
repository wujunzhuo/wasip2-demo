# wasip2-demo

https://component-model.bytecodealliance.org/language-support.html

## 1. Build wit

```sh
cargo install wkg

wkg wit fetch

wkg wit build -o wit/demo.wasm
```

## 2. Build Rust guest wasm file

```sh
rustup target add wasm32-wasip2
rustup update

cargo build --package demo_guest --target wasm32-wasip2 --release
```

https://github.com/seanmonstar/reqwest/pull/2453

## 3. Run the host program

```sh
cargo build --package demo_host --release

./target/release/demo_host target/wasm32-wasip2/release/demo_guest.wasm https://httpbin.org/uuid
```

## 4. Build JavaScript guest wasm file

```sh
npm install -g @bytecodealliance/componentize-js @bytecodealliance/jco

cd guest-js

jco componentize app.js --wit ../wit --world-name demo --out demo_guest.wasm

cd ..

./target/release/demo_host guest-js/demo_guest.wasm https://httpbin.org/uuid
```

## 5. Build Go guest wasm file

```sh
cd guest-go

go install go.bytecodealliance.org/cmd/wit-bindgen-go

wit-bindgen-go generate --world demo -o internal/ ../wit

tinygo build -o demo_guest.wasm -target=wasip2 --wit-package ../wit/demo.wasm --wit-world demo app.go

cd ..

./target/release/demo_host guest-go/demo_guest.wasm https://httpbin.org/uuid
```

## 6. Build Python guest wasm file

```sh
pip install componentize-py

cd guest-python

componentize-py --wit-path ../wit --world demo bindings .

# https://github.com/bytecodealliance/componentize-py/issues/96
componentize-py --wit-path ../wit --world demo componentize app -o demo_guest.wasm

cd ..

./target/release/demo_host guest-python/demo_guest.wasm https://httpbin.org/uuid
```
