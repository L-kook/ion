mod utils;

use std::cell::RefCell;
use std::ffi::c_void;

// #![allow(warnings)]
use ion::utils::tokio_ext::local_thread_runtime;

// use ion::values::raw::Scope;
// use ion::values::raw::Value;
use ion::*;
use tokio_util::task::TaskTracker;

pub fn main() -> anyhow::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();

    v8::V8::set_flags_from_string(
        "--no_freeze_flags_after_init --expose_gc --harmony-shadow-realm --allow_natives_syntax --turbo_fast_api_calls --js-source-phase-imports",
    );
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // utils::bench(|| main_async())?;
    // local_thread_runtime(main_async())??;
    main_async()?;

    Ok(())
}

// async fn main_async() -> anyhow::Result<()> {
fn main_async() -> anyhow::Result<()> {
    // Spawn v8 Isolate


    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let isolate_ptr = isolate.as_mut() as *mut v8::Isolate;
    // let mut states = RuntimeStateMap::default();

    {
        // Initial setup
        let handle_scope = &mut v8::HandleScope::new(unsafe { &mut *isolate_ptr });

        let mut context = Box::new(v8::Context::new(handle_scope, Default::default()));

        // let context_global = v8::Global::new(handle_scope, *context);

        let mut global_this = Box::new(v8::Global::new(
            unsafe { &mut *isolate_ptr },
            context.global(handle_scope),
        ));

        let _context_scope = v8::ContextScope::new(handle_scope, *context);
        let mut async_tasks = Box::new(TaskTracker::new());

        let mut env = Env::new(
            &mut isolate,
            &mut context,
            &mut global_this,
            &mut async_tasks,
        );

        {
            let mut scope = env.scope();
            {
                let mut scope = v8::HandleScope::new(&mut scope);
                let global_this = env.global_this();
                let global_this = global_this.open(&mut scope);

                let key = v8::String::new(&mut scope, "Hello").unwrap();
                let value = v8::String::new(&mut scope, "World").unwrap();
                Reference::register_global_finalizer(
                    value,
                    &mut env,
                    1,
                    ReferenceOwnership::Rust,
                    Some(Box::new(|_| println!("Dropped"))),
                );

                global_this.set(&mut scope, key.into(), value.into());
            };
            utils::v8_trigger_gc(isolate_ptr);

            println!("2");
            {
                let mut scope = env.scope();
                utils::v8_eval_print(&mut scope, "globalThis.Hello");
            };
            utils::v8_trigger_gc(isolate_ptr);

            {
                let mut scope = env.scope();
                utils::v8_eval_print(&mut scope, "delete globalThis.Hello");
            };
            utils::v8_trigger_gc(isolate_ptr);
            println!("3");
            drop(scope);
            utils::v8_trigger_gc(isolate_ptr);
            println!("4");    
        };
        utils::v8_trigger_gc(isolate_ptr);
        println!("5");

    }

    utils::v8_trigger_gc(isolate_ptr);
    Ok(())
}
