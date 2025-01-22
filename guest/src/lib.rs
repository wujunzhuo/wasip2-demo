use std::{
    io::{Error, Read, Write},
    net::{Shutdown, TcpStream},
};

use exports::app::demo::worker::Guest;

wit_bindgen::generate!({
    path: "../wit/demo.wasm",
    generate_all,
    world: "demo",
});

struct Worker {}

impl Guest for Worker {
    fn tcp_chat(addr: String, request: Vec<u8>) -> Result<Vec<u8>, String> {
        || -> Result<Vec<u8>, Error> {
            println!("wasm guest[tcp_chat]");

            let mut stream = TcpStream::connect(&addr)?;
            stream.write_all(&request)?;
            println!(
                "wasm guest[tcp_chat]: sent {} bytes to {}",
                request.len(),
                addr
            );

            let mut response = Vec::new();
            let n = stream.read_to_end(&mut response)?;
            println!("wasm guest[tcp_chat]: received {} bytes from {}", n, addr);

            stream.shutdown(Shutdown::Write)?;
            Ok(response)
        }()
        .map_err(|e| e.to_string())
    }
}

export!(Worker);
