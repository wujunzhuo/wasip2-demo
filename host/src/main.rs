use std::env;

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
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: demo_host wasm_file url (e.g. https://httpbin.org/uuid)");
        return Ok(());
    }
    let wasm_file = args[1].as_str();
    let url = args[2].as_str();

    let engine = Engine::default();
    let component = Component::from_file(&engine, wasm_file)?;

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

    match worker.call_http_fetch(&mut store, &url)? {
        Ok(response) => {
            println!("\nresponse:\n------------\n{response}\n------------\n");
        }
        Err(e) => {
            eprintln!("call_http_fetch error: {}", e);
        }
    }

    Ok(())
}
