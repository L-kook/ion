use ion::*;

pub fn main() -> anyhow::Result<()> {
    // Start the runtime
    let runtime = JsRuntime::initialize_once()?;

    // Create an isolate running on a dedicated thread
    let worker = runtime.spawn_worker()?;

    // Open a JavaScript context on the isolate thread to execute JavaScript on
    // You can open multiple contexts, sharing the same thread
    {
        let ctx = worker.create_context()?;

        // Execute some JavaScript in the context
        ctx.exec_blocking(|env| {
            // Evaluate arbitrary JavaScript, the result of the last line is returned
            let value = env.eval_script::<JsNumber>("1 + 1")?;

            // Cast to Rust type
            let result = value.get_u32()?;

            println!("Returned: {}", result);
            Ok(())
        })?;
    };

    Ok(())
}
