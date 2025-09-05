use std::ffi::c_void;

use tokio_util::task::TaskTracker;

#[derive(Debug, Clone, Copy)]
pub struct Env {
    isolate_ptr: *mut v8::Isolate,
    context: *mut v8::Local<'static, v8::Context>,
    global_this: *mut c_void, // v8::Global<v8::Object>,
    async_tasks: *mut TaskTracker,
    inner: *mut Env,
}

impl Env {
    pub (crate) fn new(
        mut isolate_ptr: impl AsMut<v8::Isolate>,
        mut context: impl AsMut<v8::Local<'static, v8::Context>>,
        mut global_this: impl AsMut<v8::Global<v8::Object>>,
        mut async_tasks: impl AsMut<TaskTracker>,
    ) -> Box<Self> {
        let mut env = Box::new(Env {
            isolate_ptr: isolate_ptr.as_mut() as _,
            context: context.as_mut() as _,
            global_this: global_this.as_mut() as *mut v8::Global<v8::Object> as _,
            async_tasks: async_tasks.as_mut() as _,
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
        // SAFETY: Lifetime of `Isolate` is longer than `Env`.
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
}
