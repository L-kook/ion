use clap::Parser;
use ion::*;

#[derive(Debug, Parser)]
pub struct EvalCommand {
    /// Code to evaluate
    pub code: String,
}

pub fn main(command: EvalCommand) -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    runtime.register_extension(ion::extensions::console())?;
    runtime.register_extension(ion::extensions::set_interval())?;
    runtime.register_extension(ion::extensions::set_timeout())?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(|env| {
        env.eval_script::<JsUnknown>(command.code)?;
        Ok(())
    })?;

    Ok(())
}
