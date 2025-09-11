use ion::*;

pub fn main() -> anyhow::Result<()> {
    let code = std::env::args()
        .collect::<Vec<String>>()
        .get(2)
        .cloned()
        .expect("No code provided");

    let runtime = JsRuntime::initialize_once()?;

    runtime.register_resolver(ion::resolvers::relative)?;

    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_interval())?;
    runtime.register_extension(ion::extensions::set_timeout())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.eval(code)?;
    Ok(())
}
