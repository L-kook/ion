use std::path::PathBuf;

use ion::utils::PathExt;
use ion::*;

static CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> anyhow::Result<()> {
    let entry_point = PathBuf::from(CARGO_MANIFEST_DIR)
        .join("src")
        .join("testing")
        .join("transformers")
        .join("js")
        .join("main.js");

    let runtime = JsRuntime::initialize_once()?;

    runtime.register_extension(ion::extensions::console())?;
    runtime.register_transformer(ion::transformers::json())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(move |env| env.import(entry_point.try_to_string()?))?;

    Ok(())
}
