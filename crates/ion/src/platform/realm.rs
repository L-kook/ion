use tokio_util::task::TaskTracker;

use crate::Env;

// Container that constructs a V8 context and preserves the internals until dropped
pub struct JsRealm {
    id: usize,
    env: Box<Env>,
    context: *mut v8::Local<'static, v8::Context>,
    global_this: *mut std::ffi::c_void, // v8::Global<v8::Object>,
    async_tasks: *mut TaskTracker,
    handle_scope: *mut v8::HandleScope<'static, ()>,
    context_scope: *mut v8::ContextScope<'static, v8::HandleScope<'static>>,
}

impl JsRealm {
    pub fn new(isolate_ptr: *mut v8::Isolate) -> Box<Self> {
        let handle_scope = Box::new(v8::HandleScope::new(unsafe { &mut *isolate_ptr }));
        let handle_scope_ptr = Box::into_raw(handle_scope);
        let handle_scope = unsafe { &mut *handle_scope_ptr };

        let context = Box::new(v8::Context::new(&mut *handle_scope, Default::default()));
        let context_ptr = Box::into_raw(context);
        let context = unsafe { *context_ptr };

        let global_this = Box::new(v8::Global::new(
            unsafe { &mut *isolate_ptr },
            context.global(&mut *handle_scope),
        ));
        let global_this_ptr = Box::into_raw(global_this);

        let context_scope = Box::new(v8::ContextScope::new(handle_scope, context));
        let context_scope_ptr = Box::into_raw(context_scope);

        let async_tasks = Box::new(TaskTracker::new());
        let async_tasks_ptr = Box::into_raw(async_tasks);

        let env = Env::new(isolate_ptr, context_ptr, global_this_ptr, async_tasks_ptr);

        let mut realm = Box::new(JsRealm {
            id: 0,
            env,
            context: context_ptr,
            global_this: global_this_ptr as _,
            async_tasks: async_tasks_ptr,
            handle_scope: handle_scope_ptr,
            context_scope: context_scope_ptr,
        });

        let realm_ptr = realm.as_mut() as *mut JsRealm;
        let realm_id = realm_ptr as usize;

        realm.id = realm_id;

        realm
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn async_tasks(&self) -> &TaskTracker {
        unsafe { &mut *self.async_tasks }
    }

    pub async fn drain_async_tasks(&self) {
        self.async_tasks().close();
        self.async_tasks().wait().await;
    }

    pub fn env(&self) -> &Box<Env> {
        &self.env
    }
}

impl Drop for JsRealm {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.global_this as *mut v8::Global<v8::Object>) });
        drop(unsafe { Box::from_raw(self.context_scope) });
        drop(unsafe { Box::from_raw(self.context) });
        drop(unsafe { Box::from_raw(self.handle_scope) });
        drop(unsafe { Box::from_raw(self.async_tasks) });
    }
}
