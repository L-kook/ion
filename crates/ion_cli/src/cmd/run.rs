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
    runtime.register_resolver(ion::resolvers::relative);

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.import(entry.try_to_string()?)?;
    Ok(())
}
