use std::path::PathBuf;

use clap::Parser;
use ion::utils::PathExt;
use normalize_path::NormalizePath;

#[derive(Debug, Parser)]
pub struct TestCommand {
    /// Target get file to run
    pub files: Vec<PathBuf>,
}

pub fn main(command: TestCommand) -> anyhow::Result<()> {
    let mut entries = vec![];

    let Ok(cwd) = std::env::current_dir() else {
        return Err(anyhow::anyhow!("Unable to get cwd"));
    };

    // Convert paths from relative to absolute
    for file in command.files {
        if file.is_absolute() {
            entries.push(file.normalize());
        } else {
            entries.push(cwd.join(&file).normalize());
        }
    }

    let runtime = ion::JsRuntime::initialize_once()?;

    // Resolvers
    runtime.register_resolver(ion::resolvers::relative)?;

    // Transformers
    runtime.register_transformer(ion::transformers::json())?;
    runtime.register_transformer(ion::transformers::ts())?;
    runtime.register_transformer(ion::transformers::tsx())?;

    // Extensions
    runtime.register_extension(ion::extensions::event_target())?;
    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_timeout())?;
    runtime.register_extension(ion::extensions::set_interval())?;
    runtime.register_extension(ion::extensions::test())?;
    runtime.register_extension(ion::extensions::global_this())?;

    for file in entries {
        let worker = runtime.spawn_worker()?;
        let ctx = worker.create_context()?;
        ctx.import(file.try_to_string()?)?;
    }
    Ok(())
}
