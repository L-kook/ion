use std::path::PathBuf;

use clap::Parser;
use ion::utils::PathExt;
use normalize_path::NormalizePath;

#[derive(Debug, Parser)]
pub struct RunCommand {
    /// Target get file to run
    pub path: PathBuf,
}

pub fn main(command: RunCommand) -> anyhow::Result<()> {
    let entry = if command.path.is_absolute() {
        command.path
    } else {
        let Ok(cwd) = std::env::current_dir() else {
            return Err(anyhow::anyhow!("Unable to get cwd"));
        };
        cwd.join(&command.path).normalize()
    }
    .normalize();

    let runtime = ion::JsRuntime::initialize_once()?;

    // Resolvers
    runtime.register_resolver(ion::resolvers::relative)?;

    // Transformers
    runtime.register_transformer(ion::transformers::json())?;
    runtime.register_transformer(ion::transformers::ts())?;
    runtime.register_transformer(ion::transformers::tsx())?;

    // Extensions
    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_timeout())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.import(entry.try_to_string()?)?;
    Ok(())
}
