use std::ffi::c_void;
use std::time::Duration;

use tokio::task::LocalSet;
use tokio_util::task::TaskTracker;

pub fn main() -> anyhow::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Run a current-thread Tokio runtime within a LocalSet
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on_local(main_async())
}

async fn main_async() -> anyhow::Result<()> {
    // Spawn v8 Isolate
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    {
        // Initial setup
        let handle_scope = &mut v8::HandleScope::new(&mut isolate);
        let context = v8::Context::new(handle_scope, Default::default());

        let mut context_scope_ref = Box::new(v8::ContextScope::new(handle_scope, context));
        let context_scope = context_scope_ref.as_mut();

        // State to pass into callbacks as v8::External
        let state = Box::new(State {
            // Context scope is used in async closures to
            // open scopes against the topmost scope
            context_scope: context_scope as *mut v8::ContextScope<'_, v8::HandleScope<'_>> as _,
            // Using a tokio TaskTracker to act as the "Event Loop"
            tasks: TaskTracker::new(),
        });

        // Define globals
        {
            let scope = &mut v8::HandleScope::new(context_scope);
            define_console(scope);
            define_set_timeout(scope, &state);
        }

        // Eval some code
        {
            let scope = &mut v8::HandleScope::new(context_scope);
            let code = r#"
                const sleep = d => new Promise(r => setTimeout(r, d))
                
                void async function main() {
                    console.log(`1`)
                    await sleep(1000)
                    console.log(`2`)
                    await sleep(1000)
                    console.log(`3`)
                    await sleep(1000)
                    console.log(`4`)
                    await sleep(1000)
                    console.log(`5`)
                }()
            "#;
            let code = v8::String::new(scope, code).unwrap();
            let script = v8::Script::compile(scope, code, None).unwrap();
            script.run(scope).unwrap();
        }

        // Wait for any remaining & nested async tasks to complete
        // before shutting down the context & isolate
        state.tasks.close();
        state.tasks.wait().await;
    }

    Ok(())
}

fn define_set_timeout(
    scope: &mut v8::HandleScope<'_, v8::Context>,
    state: &Box<State>,
) {
    let context = scope.get_current_context();
    let global_this = context.global(scope);

    // setTimeout
    {
        let key = v8::String::new(scope, "setTimeout").unwrap();
        let callback = |
            scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            _rv: v8::ReturnValue
        | {
            let callback = {
                let arg0 = args.get(0).try_cast::<v8::Function>().unwrap();
                // The async closure occurs in a separate scope so the callback
                // must be Global'd in order to prevent it from being GC'd
                v8::Global::new(scope, arg0)
            };

            let duration = {
                args.get(1)
                    .cast::<v8::Number>()
                    .uint32_value(scope)
                    .unwrap()
            };

            let state = {
                let data = args.data().cast::<v8::External>();
                unsafe { &*(data.value() as *const State) }
            };

            // Not needed/illustrative only. The async task is non-blocking
            // so the scope is not usable in the async closure
            #[allow(dropping_references)]
            drop(scope);

            state.tasks.clone().spawn_local(async move {
                tokio::time::sleep(Duration::from_millis(duration as _)).await;
                // Scopes don't persist across calls to .await for some reason
                {
                    // Opening a scope against root-most scope (context scope) to
                    // execute the callback with
                    let scope = &mut v8::HandleScope::new(state.context_scope());
                    let callback = v8::Local::new(scope, callback);
                    let recv = v8::undefined(scope);
                    callback.call(scope, recv.into(), &[]);
                }
            });
        };
        let external = state.as_ref() as *const State as *mut c_void;
        let external = v8::External::new(scope, external);
        let value = v8::Function::builder(callback)
            .data(external.into())
            .build(scope)
            .unwrap();

        global_this.set(scope, key.into(), value.into());
    };
}

pub fn define_console<'a>(scope: &mut v8::HandleScope<'a, v8::Context>) {
    let context = scope.get_current_context();
    let global_this = context.global(scope);

    // console
    let console = {
        let key = v8::String::new(scope, "console").unwrap();
        let value = v8::Object::new(scope);
        global_this.set(scope, key.into(), value.into());
        value
    };

    // console.log
    {
        let key = v8::String::new(scope, "log").unwrap();
        let callback = |
            scope: &mut v8::HandleScope,
            args: v8::FunctionCallbackArguments,
            _rv: v8::ReturnValue
        | {
            let arg0 = args.get(0);
            let arg0 = arg0.cast::<v8::String>();
            let arg0 = arg0.to_rust_string_lossy(scope);
            println!("{:?}", arg0);
        };
        let value = v8::Function::builder(callback).build(scope).unwrap();
        console.set(scope, key.into(), value.into());
    }
}

struct State {
    context_scope: *mut c_void,
    tasks: TaskTracker,
}

impl State {
    fn context_scope(&self) -> &mut v8::ContextScope<'static, v8::HandleScope<'static>> {
        unsafe {
            &mut *(self.context_scope as *mut v8::ContextScope<'static, v8::HandleScope<'static>>)
        }
    }
}

// Convenience methods for starting a local set
trait LocalRuntimeExt {
    fn block_on_local<F: Future>(&self, future: F) -> F::Output;
}

impl LocalRuntimeExt for tokio::runtime::Runtime {
    fn block_on_local<F: Future>(&self, future: F) -> F::Output {
        LocalSet::default().block_on(self, future)
    }
}
