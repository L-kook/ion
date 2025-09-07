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
                const sleep = d => new Promise(r => setTimeout(r, d))

                void async function main() {
                    console.log(`1`)
                    await sleep(1000)
                    console.log(`2`)
                    await sleep(1000)
                    console.log(`3`)
                    await sleep(1000)
                    console.log(`4`)
                    await sleep(1000)
                    console.log(`5`)
                }()
            "#,
        )?;

        Ok(())
    })?;
    Ok(())
}
