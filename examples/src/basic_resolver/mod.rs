use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_debug()?;

    // Add support for Json and TypeScript
    runtime.register_preprocessor(ion::preprocessor::json);
    runtime.register_preprocessor(ion::preprocessor::typescript);

    // Resolve relative paths
    runtime.register_resolver(ion::resolvers::relative);
    runtime.register_extension(ion::extensions::console());

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(|env| {
        let value = env.eval_script::<JsNumber>("1 + 1")?;
        let result = value.get_u32()?;
        println!("Returned: {}", result);
        Ok(())
    })?;

    Ok(())
}
