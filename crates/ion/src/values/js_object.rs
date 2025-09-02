use std::ffi::c_void;

use crate::Env;
use crate::FromJsRaw;
use crate::ToJsRaw;

pub struct JsObject {
    pub(self) _handle: *mut c_void,
}

impl JsObject {
    // pub fn new(env: &Env) -> crate::Result<Self> {
    //     let value = v8::Object::new(env.context_scope());
    //     Ok()
    // }

    // pub fn set_property<T>(
    //     &mut self,
    //     env: &Env,
    //     key: impl AsRef<str>,
    //     value: impl FromJsValue<T>,
    // ) {
    // }
}

impl FromJsRaw for JsObject {
    fn from_js_raw(
        _env: &Env,
        _value: v8::Local<'_, v8::Value>,
    ) -> Self {
        todo!()
    }
}

impl ToJsRaw for JsObject {
    fn into_js_raw(&self) -> v8::Local<'_, v8::Value> {
        todo!()
    }
}
