use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(Debug)]
pub struct RawIsolate(*mut v8::OwnedIsolate);

impl RawIsolate {
    pub fn new(isolate: v8::OwnedIsolate) -> Rc<Self> {
        Rc::new(Self(Box::into_raw(Box::new(isolate))))
    }

    pub fn as_mut(&self) -> &'static mut v8::Isolate {
        unsafe { &mut *self.0 as _ }
    }
}

impl Deref for RawIsolate {
    type Target = v8::OwnedIsolate;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for RawIsolate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl Drop for RawIsolate {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.0) })
    }
}
