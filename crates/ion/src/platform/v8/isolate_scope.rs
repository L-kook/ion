use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug)]
pub struct RawIsolateScope(*mut v8::HandleScope<'static, ()>);

impl RawIsolateScope {
    pub fn new(scope: v8::HandleScope<'_, ()>) -> Rc<Self> {
        Rc::new(Self(Box::into_raw(Box::new(scope)) as _))
    }

    pub fn as_mut(&self) -> &mut v8::HandleScope<'static, ()> {
        unsafe { &mut *self.0 }
    }
}

impl Deref for RawIsolateScope {
    type Target = v8::HandleScope<'static, ()>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for RawIsolateScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl Drop for RawIsolateScope {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.0) })
    }
}
