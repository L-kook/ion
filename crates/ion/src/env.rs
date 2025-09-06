use std::ffi::c_void;
use std::path::Path;

use tokio_util::task::TaskTracker;

use crate::FromJsValue;
use crate::platform::Value;

#[derive(Debug, Clone, Copy)]
pub struct Env {
    pub(crate) isolate_ptr: *mut v8::Isolate,
    pub(crate) context: *mut v8::Local<'static, v8::Context>,
    pub(crate) global_this: *mut c_void, // v8::Global<v8::Object>,
    pub(crate) async_tasks: *mut TaskTracker,
    pub(crate) inner: *mut Env,
}

impl Env {
    pub fn new(
        isolate_ptr: *mut v8::Isolate,
        context: *mut v8::Local<'static, v8::Context>,
        global_this: *mut v8::Global<v8::Object>,
        async_tasks: *mut TaskTracker,
    ) -> Box<Self> {
        let mut env = Box::new(Env {
            isolate_ptr: isolate_ptr,
            context,
            global_this: global_this as _,
            async_tasks: async_tasks,
            inner: std::ptr::null_mut(),
        });

        env.inner = env.as_mut() as *mut Env;
        env
    }

    pub fn into_raw(&self) -> *mut Env {
        self.inner
    }

    pub unsafe fn from_raw(r: *mut Env) -> Env {
        unsafe { *(r as *mut Env) }
    }

    pub fn async_tasks(&self) -> &TaskTracker {
        unsafe { &mut *self.async_tasks }
    }

    pub fn isolate(&mut self) -> &mut v8::Isolate {
        // SAFETY: Lifetime of `Isolate` is longer than `Env`.
        unsafe { &mut *self.isolate_ptr }
    }

    pub fn global_this(&self) -> v8::Global<v8::Object> {
        let v = self.global_this as *mut v8::Global<v8::Object>;
        unsafe { (*v).clone() }
    }

    pub fn context(&self) -> v8::Local<'static, v8::Context> {
        unsafe { *self.context }
    }

    pub fn scope(&self) -> v8::CallbackScope<'static> {
        // SAFETY: `v8::Local` is always non-null pointer; the `HandleScope` is
        // already on the stack, but we don't have access to it.
        let context = unsafe { &mut *self.context };
        // SAFETY: there must be a `HandleScope` on the stack, this is ensured because
        // we are in a V8 callback or the module has already opened a `HandleScope`
        // using `napi_open_handle_scope`.
        unsafe { v8::CallbackScope::new(*context) }
    }

    pub fn timeout(
        &self,
        callback: impl 'static + FnOnce(&mut v8::CallbackScope<'static>),
        duration: std::time::Duration,
    ) {
        let context = unsafe { &mut *self.context };
        self.async_tasks().spawn_local(async move {
            tokio::time::sleep(duration).await;
            let scope = &mut unsafe { v8::CallbackScope::new(*context) };
            callback(scope);
        });
    }

    pub fn timeout_ms(
        &self,
        callback: impl 'static + FnOnce(&mut v8::CallbackScope<'static>),
        duration: u64,
    ) {
        self.timeout(callback, tokio::time::Duration::from_millis(duration));
    }

    pub fn eval_script<Return: FromJsValue>(
        &self,
        code: impl AsRef<str>,
    ) -> crate::Result<Return> {
        let scope = &mut self.scope();

        let Some(code) = v8::String::new(scope, code.as_ref()) else {
            panic!();
        };

        let Some(script) = v8::Script::compile(scope, code, None) else {
            panic!();
        };

        let Some(value) = script.run(scope) else {
            panic!();
        };

        Return::from_js_value(self, Value::from(value))
    }

    /// Load a file and evaluate it
    pub fn import(
        &self,
        _path: impl AsRef<Path>,
    ) -> crate::Result<()> {
        todo!()
    }
}
