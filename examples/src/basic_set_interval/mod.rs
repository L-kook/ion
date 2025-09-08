use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_debug()?;

    // Register extensions
    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_timeout())?;
    runtime.register_extension(ion::extensions::set_interval())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(|env| {
        env.eval_script::<JsUnknown>(
            r#"
                let i = 0;

                let timerRef = setInterval(() => {
                    console.log(`${i} Interval Ran`);
                    i += 1;
                }, 100);

                setTimeout(() => {
                    console.log(`setInterval cleared`);
                    clearInterval(timerRef);
                }, 500);
            "#,
        )?;

        Ok(())
    })?;
    Ok(())
}
