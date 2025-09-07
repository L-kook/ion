#![allow(warnings)]
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
    println!("ok");
    // Spawn v8 Isolate
    // let mut isolate = v8::Isolate::new(v8::CreateParams::default());

    // {
    //     // Initial setup
    //     let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    //     let context = v8::Context::new(handle_scope, Default::default());

    //     let mut context_scope_ref = Box::new(v8::ContextScope::new(handle_scope, context));
    //     let context_scope = context_scope_ref.as_mut();

    //     context_scope.set_host_initialize_import_meta_object_callback(init_meta);

    //     // context_scope.set_host_import_module_dynamically_callback(
    //     //     |scope: &mut v8::HandleScope<'_>,
    //     //      _host_defined_options: v8::Local<'_, v8::Data>,
    //     //      resource_name: v8::Local<'_, v8::Value>,
    //     //      specifier: v8::Local<'_, v8::String>,
    //     //      import_attributes: v8::Local<'_, v8::FixedArray>|
    //     //      -> Option<v8::Local<'_, v8::Promise>> {
    //     //         let specifier_str = specifier
    //     //             .to_string(scope)
    //     //             .unwrap()
    //     //             .to_rust_string_lossy(scope);

    //     //         let referrer_name_str = resource_name
    //     //             .to_string(scope)
    //     //             .unwrap()
    //     //             .to_rust_string_lossy(scope);
    //     //         println!("foo {} {}", specifier_str, referrer_name_str);

    //     //         todo!();
    //     //     },
    //     // );

    //     // State to pass into callbacks as v8::External
    //     let state = Box::new(State {
    //         // Context scope is used in async closures to
    //         // open scopes against the topmost scope
    //         context_scope: context_scope as *mut v8::ContextScope<'_, v8::HandleScope<'_>> as _,
    //         // Using a tokio TaskTracker to act as the "Event Loop"
    //         tasks: TaskTracker::new(),
    //     });

    //     // Define globals
    //     {
    //         let scope = &mut v8::HandleScope::new(context_scope);
    //         define_console(scope);
    //         define_set_timeout(scope, &state);
    //     }

    //     // Eval some code
    //     {
    //         // let scope = &mut v8::HandleScope::new(context_scope);
    //         let scope = &mut v8::TryCatch::new(context_scope);

    //         let source = r#"
    //             // import * as foo from "foo"

    //             console.log("Module init")
    //             console.log(`${typeof import.meta}`)

    //             export const v = "str"

    //             export function main() {
    //                 console.log("Hi")
    //                 // import("./test.js")
    //             }
    //         "#;

    //         let source_string = v8::String::new(scope, source).unwrap();

    //         let resource_name = v8::String::new(scope, "main.js").unwrap().into();
    //         let origin = v8::ScriptOrigin::new(
    //             scope,
    //             resource_name,
    //             0,
    //             0,
    //             false,
    //             0,
    //             None,
    //             false,
    //             false,
    //             true,
    //             None,
    //         );
    //         let mut source = v8::script_compiler::Source::new(source_string, Some(&origin));
    //         let program = v8::script_compiler::compile_module(scope, &mut source).unwrap();

    //         let res = program
    //             .instantiate_module(
    //                 scope,
    //                 |context: v8::Local<'_, v8::Context>,
    //                  specifier: v8::Local<'_, v8::String>,
    //                  import_attributes: v8::Local<'_, v8::FixedArray>,
    //                  referrer: v8::Local<'_, v8::Module>|
    //                  -> Option<v8::Local<'_, v8::Module>> {
    //                     // let scope = &mut unsafe { v8::CallbackScope::new(context) };

    //                     // println!("specifier {}", specifier.to_rust_string_lossy(scope));
    //                     // println!("referrer {:?}", referrer.get_identity_hash());
    //                     println!("instantiate_module called");
    //                     None
    //                 },
    //             )
    //             .unwrap();

    //         let promise = program.evaluate(scope).unwrap().cast::<v8::Promise>();

    //         scope.perform_microtask_checkpoint();

    //         promise.result(scope);

    //         let arr = program
    //             .get_module_namespace()
    //             .cast::<v8::Object>()
    //             .get_property_names(scope, Default::default())
    //             .unwrap();

    //         println!("len {}", arr.length());
    //         for i in 0..arr.length() {
    //             let i = v8::Number::new(scope, i as _).into();
    //             println!(
    //                 "ley {:?}",
    //                 arr.get(scope, i).unwrap().to_rust_string_lossy(scope)
    //             );
    //         }

    //         // println!("{:?}", );
    //         // println!("{:?}", res.get(scope, key).unwrap().cast::<v8::String>().to_rust_string_lossy(scope));
    //         // let script = v8::Script::compile(scope, code, None).unwrap();
    //         // script.run(scope).unwrap();
    //     }

    //     // Wait for any remaining & nested async tasks to complete
    //     // before shutting down the context & isolate
    //     state.tasks.close();
    //     state.tasks.wait().await;
    // }

    Ok(())
}

unsafe extern "C" fn init_meta(
    context: v8::Local<v8::Context>,
    module: v8::Local<v8::Module>,
    meta: v8::Local<v8::Object>,
) {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };

    {
        let key = v8::String::new(scope, "url").unwrap();
        let value = v8::String::new(scope, "something").unwrap();
        meta.create_data_property(scope, key.into(), value.into());
    }

    println!("meta called")
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
        let callback = |scope: &mut v8::HandleScope,
                        args: v8::FunctionCallbackArguments,
                        _rv: v8::ReturnValue| {
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
        let callback = |scope: &mut v8::HandleScope,
                        args: v8::FunctionCallbackArguments,
                        _rv: v8::ReturnValue| {
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
    fn block_on_local<F: Future>(
        &self,
        future: F,
    ) -> F::Output;
}

impl LocalRuntimeExt for tokio::runtime::Runtime {
    fn block_on_local<F: Future>(
        &self,
        future: F,
    ) -> F::Output {
        LocalSet::default().block_on(self, future)
    }
}
