use std::rc::Rc;
use std::time::Duration;

// #![allow(warnings)]
use ion::utils::tokio_ext::local_thread_runtime;

// use ion::values::raw::Scope;
// use ion::values::raw::Value;
use ion::*;
use tokio_util::task::TaskTracker;

pub fn main() -> anyhow::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();
    
    v8::V8::set_flags_from_string("--no_freeze_flags_after_init --expose_gc --harmony-shadow-realm --allow_natives_syntax --turbo_fast_api_calls --js-source-phase-imports");
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    main_async()
    // Run a current-thread Tokio runtime within a LocalSet
    // local_thread_runtime(main_async())?
}

fn main_async() -> anyhow::Result<()> {
    // Spawn v8 Isolate
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let isolate_ptr = isolate.as_mut() as *mut v8::Isolate;
    // let mut states = RuntimeStateMap::default();

    // Initial setup
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    
    let context = v8::Context::new(handle_scope, Default::default());
    let context_global = v8::Global::new(unsafe { &mut *isolate_ptr}, context);

    let global_this = v8::Global::new(unsafe { &mut *isolate_ptr}, context.global(handle_scope));
    
    let context_scope = v8::ContextScope::new(handle_scope, context);
    let async_tasks = TaskTracker::new();
    

    let env = Env {
        isolate_ptr,
        context: context_global.into_raw(),
        open_handle_scopes: 0,
        global_this,
        async_tasks,
    };

    println!("hi");

    // let data = 42;
    // let external = JsExternal::new(&env, data)?;

    // let x = env.fork(|env| {
    //     let mut global_this = env.global_this()?;.
    //     let value = JsString::new(&env, "Hello")?;

    //     let weak = v8::Weak::with_guaranteed_finalizer(env.scope.current(), value.value().inner(), Box::new(|| {
    //         println!("dropped")
    //     }));

    //     let l = weak.to_local(env.scope.current()).unwrap();

    //     global_this.set_named_property("foo", JsString::from_js_value(&env, Value::from(l))?)?;
    //     Ok(l)
    // })?;

    // env.scope.current().request_garbage_collection_for_testing(v8::GarbageCollectionType::Full);
    // eval_print(&env, "globalThis.foo");

    // env.fork(|env| {
    //     let mut global_this = env.global_this()?;
    //     global_this.delete_named_property("foo")?;
    //     Ok(())
    // })?;

    // drop(x);
    // env.scope.current().request_garbage_collection_for_testing(v8::GarbageCollectionType::Full);
    // eval_print(&env, "globalThis.foo");


        // let scope = env.scope.current();
        // let global_this = scope.get_current_context().global(scope);

        // let key = v8::String::new(scope, "foo").unwrap();
        // global_this.set(scope, key.into(), external.clone(&env).value().inner());

        // let value = eval_script(env.scope.current(), "(ext) => {}")?.cast::<v8::Function>();

        // let _v = external.clone(&env);
        // let external_l = external.value().inner();
        // let func = v8::Function::builder(|
        //     scope: &mut v8::HandleScope,
        //  args: v8::FunctionCallbackArguments,
        //  rv: v8::ReturnValue
        //     | {
        //         println!("Called");
        //     }).data(external_l).build(scope).unwrap();


        // eval_script(&env.scope, "globalThis.foo = (ext) => {}")?;
        // eval_print(&env.scope, "JSON.stringify(globalThis.foo)");
        
        // println!("{}", external.as_inner()?);
        
        
        // states.insert(state);
    // isolate.request_garbage_collection_for_testing(v8::GarbageCollectionType::Full);

    // println!("asd");
    Ok(())
}

// struct RuntimeState {
//     context_scope: *mut c_void,
//     tasks: TaskTracker,
// }

// impl RuntimeState {
//     fn context_scope(&self) -> &mut v8::ContextScope<'static, v8::HandleScope<'static>> {
//         unsafe {
//             &mut *(self.context_scope as *mut v8::ContextScope<'static, v8::HandleScope<'static>>)
//         }
//     }
// }

// #[derive(Default)]
// struct RuntimeStateMap {
//     map: HashMap<usize, RuntimeState>,
//     counter: usize,
// }

// impl RuntimeStateMap {
//     pub fn insert(
//         &mut self,
//         state: RuntimeState,
//     ) -> usize {
//         let id = self.counter;
//         self.counter += 1;
//         self.map.insert(id, state);
//         id
//     }

//     pub fn get(
//         &self,
//         id: &usize,
//     ) -> anyhow::Result<&RuntimeState> {
//         let Some(state) = self.map.get(id) else {
//             anyhow::bail!("")
//         };
//         Ok(state)
//     }

//     pub fn remove(
//         &mut self,
//         id: &usize,
//     ) -> anyhow::Result<RuntimeState> {
//         let Some(state) = self.map.remove(id) else {
//             anyhow::bail!("")
//         };
//         Ok(state)
//     }
// }

// pub fn eval_script<'a, S: AsRef<str>>(
//     env:  &Env,
//     code: S,
// ) -> crate::Result<v8::Local<'a, v8::Value>> {
//     let Some(code) = v8::String::new(env.scope.current(), code.as_ref()) else {
//         panic!();
//     };
//     let Some(script) = v8::Script::compile(env.scope.current(), code, None) else {
//         panic!();
//     };
//     let Some(value) = script.run(env.scope.current()) else {
//         panic!();
//     };
//     Ok(value)
// }

// fn eval_print(env: &Env, code: impl AsRef<str>) {
//     let result = eval_script(env, code).unwrap();
//     println!("Result: {}", result.to_rust_string_lossy(env.scope.current()));
// }


pub fn main() -> anyhow::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();

    v8::V8::set_flags_from_string(
        "--no_freeze_flags_after_init --expose_gc --harmony-shadow-realm --allow_natives_syntax --turbo_fast_api_calls --js-source-phase-imports",
    );
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    for i in 0..50 {
        println!("loop {}", i);
        for _ in 0..1000 {
            main_async()?;
        }
        // std::thread::sleep(std::time::Duration::from_millis(250));
    }

    println!("done");
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
    // Run a current-thread Tokio runtime within a LocalSet
    // local_thread_runtime(main_async())?
}















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

    // mem::bench(|| main_async())?;
    local_thread_runtime(main_async())??;
    // main_async()?;

    Ok(())
}

async fn main_async() -> anyhow::Result<()> {
    // fn main_async() -> anyhow::Result<()> {
    // Spawn v8 Isolate
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let isolate_ptr = isolate.as_mut() as *mut v8::Isolate;
    // let mut states = RuntimeStateMap::default();

    {
    // Initial setup
    let handle_scope = &mut v8::HandleScope::new(unsafe { &mut *isolate_ptr });

    let mut context = Box::new(v8::Context::new(handle_scope, Default::default()));
    let context_ptr = context.as_mut() as *mut v8::Local<'static, v8::Context>;

    // let context_global = v8::Global::new(handle_scope, *context);

    let global_this = v8::Global::new(unsafe { &mut *isolate_ptr }, context.global(handle_scope));

    let _context_scope = v8::ContextScope::new(handle_scope, *context);
    let async_tasks = TaskTracker::new();

    let mut env = Box::new(Env {
        isolate_ptr,
        context: context_ptr,
        // global_this,
        // async_tasks,
    });

    let reff = {
        let mut scope = env.scope();

        let value = v8::String::new(&mut scope, "bar").unwrap();
        let mut reff = Reference::new(
            value.cast(),
            env.as_mut() as _,
            1,
            ReferenceOwnership::Rust,
            Box::new(|env| println!("dropped")),
        );

        utils::v8_trigger_gc(isolate_ptr);
        reff.dec_ref();
        utils::v8_trigger_gc(isolate_ptr);
        reff
    };

    // let mut reff = {
    //     let scope = &mut env.scope();
    //     let context = scope.get_current_context();
    //     let global_this = context.global(scope);

    //     let key = v8::String::new(scope, "foo").unwrap();
    //     let value = v8::String::new(scope, "bar").unwrap();
    //     let reff = Reference::new(
    //         value.into(),
    //         env.as_mut() as _,
    //         1,
    //         ReferenceOwnership::Rust,
    //         Box::new(|env| println!("dropped")),
    //     );

    //     global_this.set(scope, key.into(), value.into());
    //     reff
    // };

    // utils::v8_trigger_gc(isolate_ptr);

    // {
    //     let scope = &mut env.scope();
    //     let context = scope.get_current_context();
    //     let global_this = context.global(scope);

    //     let key = v8::String::new(scope, "foo").unwrap();

    //     global_this.delete(scope, key.into());
    // };
    // reff.dec_ref();

    // utils::v8_trigger_gc(isolate_ptr);

    // {
    //     let scope = &mut env.scope();
    //     utils::v8_eval_print(scope, "globalThis.foo");
    // }

    // env.async_tasks.close();
    // env.async_tasks.wait().await;

    utils::v8_trigger_gc(isolate_ptr);
}
    Ok(())
}

#[derive(Debug)]
enum ReferenceState {
    Strong(v8::Global<v8::Value>),
    Weak(v8::Weak<v8::Value>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ReferenceOwnership {
    Runtime,
    Rust,
}

struct Reference {
    env: *mut Env,
    ref_count: u32,
    state: ReferenceState,
    ownership: ReferenceOwnership,
    finalize_cb: Option<Box<dyn 'static + FnOnce(Env)>>,
}

impl Reference {
    fn new(
        value: v8::Local<v8::Value>,
        env: *mut Env,
        initial_ref_count: u32,
        ownership: ReferenceOwnership,

        finalize_cb: Box<dyn 'static + FnOnce(Env)>,
    ) -> Box<Self> {
        let isolate = unsafe { &mut (*env).scope() };

        let mut reference = Box::new(Reference {
            env,
            state: ReferenceState::Strong(v8::Global::new(isolate, value)),
            ref_count: initial_ref_count,
            ownership,
            finalize_cb: Some(finalize_cb),
        });

        if initial_ref_count == 0 {
            reference.set_weak();
        }

        reference
    }

    fn inc_ref(&mut self) -> u32 {
        self.ref_count += 1;
        if self.ref_count == 1 {
            self.set_strong();
        }
        self.ref_count
    }

    fn dec_ref(&mut self) -> u32 {
        let old_ref_count = self.ref_count;
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        if old_ref_count == 1 && self.ref_count == 0 {
            self.set_weak();
        }
        self.ref_count
    }

    fn set_strong(&mut self) {
        if let ReferenceState::Weak(w) = &self.state {
            let isolate = unsafe { &mut (*self.env).scope() };
            if let Some(g) = w.to_global(isolate) {
                self.state = ReferenceState::Strong(g);
            }
        }
    }

    fn set_weak(&mut self) {
        let reference = self as *mut Reference;
        if let ReferenceState::Strong(g) = &self.state {
            println!("1 {}", self.ref_count);
            let cb = Box::new(move || Reference::weak_callback(reference));
            let isolate = unsafe { &mut (*self.env).scope() };
            self.state = ReferenceState::Weak(v8::Weak::with_guaranteed_finalizer(isolate, g, cb));
        }
    }

    fn weak_callback(reference: *mut Reference) {
        println!("weak_callback");

        let reference = unsafe { &mut *reference };

        let finalize_cb = &mut reference.finalize_cb;
        let ownership = reference.ownership;
        if let Some(finalize_cb) = finalize_cb.take() {
            finalize_cb(unsafe { *reference.env });
        }
    }
}
