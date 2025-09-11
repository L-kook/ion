use std::path::PathBuf;

use ion::utils::PathExt;
use ion::*;

static CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_debug()?;

    // Add a custom resolver
    runtime.register_resolver(async |ctx: ResolverContext| {
        println!("Custom Resolver Has Run For Path {:?}", ctx.from);
        Ok(None)
    })?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    let entry_point = PathBuf::from(CARGO_MANIFEST_DIR)
        .join("js")
        .join("modules")
        .join("index.js")
        .try_to_string()?;

    ctx.import(&entry_point)?;

    Ok(())
}
