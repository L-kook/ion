use std::path::PathBuf;

use ion::utils::PathExt;
use ion::*;
use normalize_path::NormalizePath;

pub fn main() -> anyhow::Result<()> {
    let file_path = std::env::args()
        .collect::<Vec<String>>()
        .get(2)
        .cloned()
        .expect("No filepath provided");

    let file_path = PathBuf::from(file_path);
    let file_path = if file_path.is_absolute() {
        file_path
    } else {
        let Ok(cwd) = std::env::current_dir() else {
            return Err(anyhow::anyhow!("Unable to get cwd"));
        };
        cwd.join(&file_path)
    }
    .normalize();

    let runtime = JsRuntime::initialize_once()?;

    runtime.register_resolver(ion::resolvers::relative)?;

    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_interval())?;
    runtime.register_extension(ion::extensions::set_timeout())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.import(file_path.try_to_string()?)?;

    Ok(())
}
