use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

#[derive(Debug)]
pub struct Scope(*mut c_void, u8, RefCounter);

impl Clone for Scope {
    fn clone(&self) -> Self {
        self.2.inc();
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl Scope {
    pub fn current(&self) -> &mut v8::HandleScope<'static, v8::Context> {
        unsafe { &mut *(self.0 as *mut v8::HandleScope<'static, v8::Context>) }
    }

    pub fn active_as_context(&self) -> &mut v8::ContextScope<'_, v8::HandleScope<'_>> {
        unsafe { &mut *(self.0 as *mut v8::ContextScope<'_, v8::HandleScope<'_>>) }
    }

    pub fn open_scope(&self) -> Scope {
        match self.1 {
            1 => Scope::from(v8::HandleScope::new(self.current())),
            2 => Scope::from(v8::HandleScope::new(self.active_as_context())),
            _ => panic!(),
        }
    }
}

impl From<v8::HandleScope<'_, v8::Context>> for Scope {
    fn from(value: v8::HandleScope<'_, v8::Context>) -> Self {
        Self(Box::into_raw(Box::new(value)) as _, 0, RefCounter::new(1))
    }
}

impl From<v8::ContextScope<'_, v8::HandleScope<'_>>> for Scope {
    fn from(value: v8::ContextScope<'_, v8::HandleScope<'_>>) -> Self {
        Self(Box::into_raw(Box::new(value)) as _, 1, RefCounter::new(1))
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        if !self.2.dec() {
            return;
        }
        match self.1 {
            0 => drop(unsafe { Box::from_raw(self.0 as *mut v8::HandleScope<'_, v8::Context>) }),
            1 => drop(unsafe {
                Box::from_raw(self.0 as *mut v8::ContextScope<'_, v8::HandleScope<'_>>)
            }),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Value(*mut c_void, RefCounter);

impl Clone for Value {
    fn clone(&self) -> Self {
        self.1.inc();
        Self(self.0.clone(), self.1.clone())
    }
}

impl Value {
    pub fn as_inner(&self) -> &mut v8::Local<'_, v8::Value> {
        unsafe { &mut *(self.0 as *mut v8::Local<'_, v8::Value>) }
    }

    pub fn inner(&self) -> v8::Local<'_, v8::Value> {
        *self.as_inner()
    }
}

impl From<v8::Local<'_, v8::Value>> for Value {
    fn from(value: v8::Local<'_, v8::Value>) -> Self {
        Self(Box::into_raw(Box::new(value)) as _, RefCounter::new(1))
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if !self.1.dec() {
            return;
        }
        drop(unsafe { Box::from_raw(self.0 as *mut v8::Local<'static, v8::Value>) })
    }
}

#[derive(Debug, Clone)]
struct RefCounter(Rc<RefCell<usize>>);

impl RefCounter {
    fn new(start: usize) -> Self {
        Self(Rc::new(RefCell::new(start)))
    }

    fn inc(&self) {
        let mut count = self.0.borrow_mut();
        (*count) += 1;
    }

    fn dec(&self) -> bool {
        let mut count = self.0.borrow_mut();
        (*count) -= 1;
        *count == 0
    }
}
