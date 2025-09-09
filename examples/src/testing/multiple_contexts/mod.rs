use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_debug()?;

    let worker = runtime.spawn_worker()?;

    let ctx0 = worker.create_context()?;
    let ctx1 = worker.create_context()?;

    drop(ctx1);
    drop(ctx0);

    Ok(())
}
