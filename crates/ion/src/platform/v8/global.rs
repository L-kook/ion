use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::platform::v8::RawContextScope;

use super::RawContext;

#[derive(Debug)]
pub struct RawGlobal(*mut v8::Local<'static, v8::Object>);

impl RawGlobal {
    pub fn new(
        context: &RawContext,
        scope: &RawContextScope,
    ) -> Rc<Self> {
        let scope = scope.as_mut();
        // Note: [`v8::Global::into_raw`] appears to have a memory leak
        let global_local = context.global(scope);
        let global_global = v8::Global::new(scope, global_local);
        let global_this = Box::into_raw(Box::new(global_global));
        Rc::new(Self(global_this as *mut v8::Local<'static, v8::Object>))
    }

    pub fn as_inner(&self) -> v8::Local<'static, v8::Object> {
        unsafe { *self.0 }
    }
}

impl Deref for RawGlobal {
    type Target = v8::Local<'static, v8::Object>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for RawGlobal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl Drop for RawGlobal {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.0 as *mut v8::Global<v8::Object>) })
    }
}
