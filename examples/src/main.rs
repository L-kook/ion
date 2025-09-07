#![allow(warnings)]
mod basic;
mod basic_async;
mod basic_resolver;
mod basic_set_timeout;
mod extensions_basic;
mod http_server;

fn main() -> anyhow::Result<()> {
    let example = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .cloned()
        .unwrap_or("basic".to_string());

    match example.as_str() {
        "basic" => basic::main(),
        "basic_async" => basic_async::main(),
        "basic_set_timeout" => basic_set_timeout::main(),
        "basic_resolver" => basic_resolver::main(),
        "extensions_basic" => extensions_basic::main(),
        "http_server" => http_server::main(),
        _ => Err(anyhow::anyhow!("No example for: \"{}\"", example)),
    }
}
