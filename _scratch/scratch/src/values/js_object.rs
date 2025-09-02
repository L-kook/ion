use std::rc::Rc;

use crate::env::Env;
use crate::raw::Scope;
use crate::raw::Value;
use crate::values::FromJsValue;
use crate::values::JsObjectValue;
use crate::values::JsValue;
pub struct JsObject {
    value: Value,
    scope: Scope,
}

impl JsObject {
    pub fn new(env: &Env) -> crate::Result<Self> {
        let scope = env.scope.current();
        let object = v8::Object::new(scope);
        Ok(Self {
            value: Value::from(object.cast::<v8::Value>()),
            scope: env.scope.clone(),
        })
    }
}

impl FromJsValue for JsObject {
    fn from_js_value(
        env: &Env,
        value: Value,
    ) -> crate::Result<Self> {
        Ok(Self {
            value,
            scope: env.scope.clone(),
        })
    }
}

impl JsValue for JsObject {
    fn value(&self) -> Value {
        self.value.clone()
    }

    fn scope(&self) -> Scope {
        self.scope.clone()
    }
}

impl JsObjectValue for JsObject {}
