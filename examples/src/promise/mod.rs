use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_timeout())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    // Execute some JavaScript in the context
    ctx.exec_blocking(|env| {
        // Evaluate arbitrary JavaScript, the result of the last line is returned
        let value = env.eval_script::<JsPromise>(
            r#"
            console.log("[JS] Promise Started");

            new Promise((resolve) => setTimeout(() => {
                console.log("[JS] Promise Resolved");
                resolve(42);
            }, 3_000));
        "#,
        )?;

        // Cast to Rust type
        value.settled::<JsNumber>(|_env, result| {
            match result {
                JsPromiseResult::Resolved(resolved) => {
                    println!("[Rust] Got {}", resolved.get_u32()?)
                }
                JsPromiseResult::Rejected(_) => unreachable!(),
            };
            Ok(())
        })?;

        println!("Exec Complete (Not Blocked)");

        Ok(())
    })?;

    Ok(())
}
