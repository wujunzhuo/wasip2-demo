# wasip2-demo

## 1. Build wasm file

```sh
rustup target add wasm32-wasip2

cargo build --package demo_guest --target wasm32-wasip2
```

## 2. Build the host program

```sh
cargo build --package demo_host
```

## 3. Run

```sh
./target/debug/demo_host

./target/debug/demo_host http://baidu.com
```
