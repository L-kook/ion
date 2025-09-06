#![allow(warnings)]
mod utils;

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use ion::utils::tokio_ext::local_thread_runtime;

// use ion::values::raw::Scope;
// use ion::values::raw::Value;
use ion::*;
use tokio_util::task::TaskTracker;

pub fn main() -> anyhow::Result<()> {
    // let platform = v8::new_default_platform(0, false).make_shared();

    // v8::V8::set_flags_from_string(
    //     "--no_freeze_flags_after_init --expose_gc --harmony-shadow-realm --allow_natives_syntax --turbo_fast_api_calls --js-source-phase-imports",
    // );
    // v8::V8::initialize_platform(platform);
    // v8::V8::initialize();

    utils::bench(|| main_async())?;
    // local_thread_runtime(main_async())??;
    // main_async()?;

    Ok(())
}

// async fn main_async() -> anyhow::Result<()> {
fn main_async() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    // Create an isolate running on a dedicated thread
    let worker = runtime.spawn_worker()?;

    // // Open a JavaScript context on the isolate thread to execute JavaScript on
    // // You can open multiple contexts, sharing the same thread
    let ctx = worker.create_context()?;

    // Execute some JavaScript in the context
    ctx.exec_blocking(|env| {
        // Evaluate arbitrary JavaScript, the result of the last line is returned
        let value = env.eval_script::<JsNumber>("1 + 1")?;

        // Cast to Rust type
        let result = value.get_u32()?;

        // println!("Returned: {}", result);
        Ok(())
    })?;

    Ok(())
}
