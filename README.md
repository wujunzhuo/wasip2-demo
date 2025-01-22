# wasip2-demo

## 1. Build wit

```sh
cargo install wkg

wkg wit fetch

wkg wit build -o wit/demo.wasm
```

## 2. Build the guest wasm file

```sh
rustup target add wasm32-wasip2

cargo build --package demo_guest --target wasm32-wasip2 --release

ln -sf target/wasm32-wasip2/release/demo_guest.wasm demo_guest.wasm
```

## 3. Build the host program

```sh
cargo build --package demo_host --release
```

## 4. Run

```sh
./target/release/demo_host

./target/release/demo_host http://baidu.com
```

## 5. Build the guest wasm file from Go

```sh
cd guest-go

go install go.bytecodealliance.org/cmd/wit-bindgen-go

wit-bindgen-go generate -o internal/ ../wit/demo.wasm

tinygo build -o demo_guest.wasm -target=wasip2 --wit-package ../wit --wit-world demo main.go

cd ..

ln -sf guest-go/demo_guest.wasm demo_guest.wasm
```

## 6. Build the guest wasm file from Python

```sh
pip install componentize-py

cd guest-python

componentize-py --wit-path ../wit --world demo bindings .

componentize-py --wit-path ../wit --world demo componentize app -o demo_guest.wasm

cd ..

ln -sf guest-python/demo_guest.wasm demo_guest.wasm
```
