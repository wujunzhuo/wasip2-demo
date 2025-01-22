use exports::app::demo::worker::Guest;

wit_bindgen::generate!({
    path: "../wit",
    generate_all,
    world: "demo",
});

struct Worker {}

impl Guest for Worker {
    fn http_fetch(url: String) -> Result<String, String> {
        || -> anyhow::Result<String> {
            println!("wasm guest-rust http-fetch: {url}");
            Ok(ureq::get(&url).call()?.body_mut().read_to_string()?)
        }()
        .map_err(|e| e.to_string())
    }
}

export!(Worker);
