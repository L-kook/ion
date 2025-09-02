#![allow(warnings)]
mod env;
mod raw;
mod result;
mod values;

use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;

use ion::utils::tokio_ext::local_thread_runtime;
use tokio_util::task::TaskTracker;

use self::env::*;
use self::raw::*;
use self::result::*;
use self::values::*;

pub fn main() -> anyhow::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Run a current-thread Tokio runtime within a LocalSet
    local_thread_runtime(main_async())?
}

async fn main_async() -> anyhow::Result<()> {
    // Spawn v8 Isolate
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let mut states = RuntimeStateMap::default();

    {
        // Initial setup
        let handle_scope = &mut v8::HandleScope::new(&mut isolate);
        let context = v8::Context::new(handle_scope, Default::default());

        let mut context_scope = Scope::from(v8::ContextScope::new(handle_scope, context));

        // let state = RuntimeState {
        //     context_scope: context_scope as *mut v8::ContextScope<'_, v8::HandleScope<'_>> as _,
        //     tasks: TaskTracker::new(),
        // };

        {
            let scope = context_scope.open_scope();
            let env = Env { scope };

            {
                let mut global_this = JsObject::from_js_value(
                    &env,
                    Value::from(context.global(env.scope.current()).cast::<v8::Value>()),
                )?;

                let key = JsString::new(&env, "foo")?;
                let value = JsString::new(&env, "bar")?;

                // let key = key.into_inner();
                // dbg!(&key);

                global_this.set_property(key, value);
                // global_this.set(env.scope.current(), key.value().inner(), value.value().inner());

                eval_print(&env.scope, "JSON.stringify(globalThis.foo)");
            }

            // let env = Env {
            //     scope: Rc::new(Scope::from(scope)),
            // };

            // dbg!(&env);

            // let mut object = JsObject::new(&env)?;

            // {
            //     let key = JsString::new(&env, "Hello")?;
            //     let value = JsString::new(&env, "World")?;
            //     object.set_property(key, value)?;
            // }

            // {
            //     // dbg!(&env);
            //     let mut object = JsObject::new(&env)?;

            //     let key = JsString::new(&env, "foo")?;
            //     let value = JsString::new(&env, "World")?;
            //     object.set_property(key, value)?;

            //     let scope = env.scope.as_inner();
            //     let global_this = context.global(scope);

            //     // let key = key.raw().into_inner();
            //     // // let value = v8::String::new(scope, "bar").unwrap();

            //     let key = v8::String::new(scope, "foo").unwrap();
            //     let value = object.raw().into_inner();
            //     global_this.set(scope, key.into(), value.into());

            //     let result = eval_script(scope, "JSON.stringify(globalThis.foo)")?;
            //     println!("Result: {}", result.to_rust_string_lossy(scope));
            // }

            // object.set_property(key, value)
        }

        // states.insert(state);
    };

    println!("asd");
    Ok(())
}

struct RuntimeState {
    context_scope: *mut c_void,
    tasks: TaskTracker,
}

impl RuntimeState {
    fn context_scope(&self) -> &mut v8::ContextScope<'static, v8::HandleScope<'static>> {
        unsafe {
            &mut *(self.context_scope as *mut v8::ContextScope<'static, v8::HandleScope<'static>>)
        }
    }
}

#[derive(Default)]
struct RuntimeStateMap {
    map: HashMap<usize, RuntimeState>,
    counter: usize,
}

impl RuntimeStateMap {
    pub fn insert(
        &mut self,
        state: RuntimeState,
    ) -> usize {
        let id = self.counter;
        self.counter += 1;
        self.map.insert(id, state);
        id
    }

    pub fn get(
        &self,
        id: &usize,
    ) -> anyhow::Result<&RuntimeState> {
        let Some(state) = self.map.get(id) else {
            anyhow::bail!("")
        };
        Ok(state)
    }

    pub fn remove(
        &mut self,
        id: &usize,
    ) -> anyhow::Result<RuntimeState> {
        let Some(state) = self.map.remove(id) else {
            anyhow::bail!("")
        };
        Ok(state)
    }
}

pub fn eval_script<'a, S: AsRef<str>>(
    scope: &mut v8::HandleScope<'a, v8::Context>,
    code: S,
) -> crate::Result<v8::Local<'a, v8::Value>> {
    let Some(code) = v8::String::new(scope, code.as_ref()) else {
        panic!();
    };
    let Some(script) = v8::Script::compile(scope, code, None) else {
        panic!();
    };
    let Some(value) = script.run(scope) else {
        panic!();
    };
    Ok(value)
}

fn eval_print(
    scope: &Scope,
    code: impl AsRef<str>,
) {
    let result = eval_script(scope.current(), code).unwrap();
    println!("Result: {}", result.to_rust_string_lossy(scope.current()));
}

fn v8_string(
    scope: &Scope,
    s: impl AsRef<str>,
) -> v8::Local<'_, v8::Value> {
    v8::String::new(scope.current(), s.as_ref())
        .unwrap()
        .cast::<v8::Value>()
}
