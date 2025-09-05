pub fn main() -> anyhow::Result<()> {
    // let runtime = ion::platform::initialize_once()?;
    // let worker = runtime.spawn_worker()?;

    // {
    //     let ctx = worker.create_context()?;

    //     ctx.exec_blocking(|env| {
    //         ion::exts::define_console(&env);
    //         ion::exts::define_set_timeout(&env);
    //         Ok(())
    //     })?;

    //     ctx.exec_blocking(|env| {
    //         env.eval_script(
    //             r#"
    //             const sleep = d => new Promise(r => setTimeout(r, d))

    //             void async function main() {
    //                 console.log(`1`)
    //                 await sleep(1000)
    //                 console.log(`2`)
    //                 await sleep(1000)
    //                 console.log(`3`)
    //                 await sleep(1000)
    //                 console.log(`4`)
    //                 await sleep(1000)
    //                 console.log(`5`)
    //             }()
    //         "#,
    //         )?;

    //         Ok(())
    //     })?;
    // };

    Ok(())
}
