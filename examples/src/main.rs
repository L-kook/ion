mod basic;
mod basic_async;
mod basic_set_timeout;
mod extensions_basic;
mod extensions_structured;
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
        "extensions_basic" => extensions_basic::main(),
        "extensions_structured" => extensions_structured::main(),
        "http_server" => http_server::main(),
        _ => Err(anyhow::anyhow!("No example for: \"{}\"", example)),
    }
}
