use std::env;

use anyhow::{anyhow, bail};
use url::Url;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

bindgen!({
    path: "../wit",
    world: "demo",
});

struct MyState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl WasiHttpView for MyState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, "demo_guest.wasm")?;

    let state = MyState {
        wasi: WasiCtxBuilder::new()
            .inherit_env()
            .inherit_stdout()
            .inherit_stderr()
            .inherit_network()
            .allow_ip_name_lookup(true)
            .allow_tcp(true)
            .allow_udp(true)
            .build(),
        http: WasiHttpCtx::new(),
        table: ResourceTable::new(),
    };

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_sync(&mut linker)?;
    let mut store = Store::new(&engine, state);

    let binding = Demo::instantiate(&mut store, &component, &linker)?;
    let worker = binding.app_demo_worker();

    let args: Vec<String> = env::args().collect();
    let url = Url::parse(match args.len() {
        1 => "http://httpbin.org/uuid",
        _ => args[1].as_str(),
    })?;
    if url.scheme() != "http" {
        bail!("only http is supported");
    }
    let host = url.host_str().ok_or(anyhow!("no host"))?;
    let port = url.port().unwrap_or(80);
    let addr = format!("{host}:{port}");
    let path = url.path();
    let request = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\nAccept: */*\r\n\r\n");
    println!("addr: {}\nrequest:\n------\n{}\n------\n", addr, request);

    match worker.call_tcp_chat(&mut store, &addr, &request.into_bytes())? {
        Ok(response) => {
            let response = String::from_utf8_lossy(&response);
            println!("\nresponse:\n------\n{}\n------\n", response);
        }
        Err(e) => {
            eprintln!("call_tcp_chat error: {}", e);
        }
    }

    Ok(())
}
