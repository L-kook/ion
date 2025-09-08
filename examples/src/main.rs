mod basic;
mod basic_async;
mod custom_extension;
mod custom_resolver;
mod deferred;
mod http_server;
mod set_interval;
mod set_timeout;
mod thread_safe_function;

fn main() -> anyhow::Result<()> {
    let example = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .cloned()
        .unwrap_or("basic".to_string());

    match example.as_str() {
        "basic" => basic::main(),
        "basic_async" => basic_async::main(),
        "set_timeout" => set_timeout::main(),
        "custom_resolver" => custom_resolver::main(),
        "set_interval" => set_interval::main(),
        "deferred" => deferred::main(),
        "custom_extension" => custom_extension::main(),
        "http_server" => http_server::main(),
        "thread_safe_function" => thread_safe_function::main(),
        _ => Err(anyhow::anyhow!("No example for: \"{}\"", example)),
    }
}
