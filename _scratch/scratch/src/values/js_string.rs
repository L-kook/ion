use std::rc::Rc;

use crate::env::Env;
use crate::raw::Scope;
use crate::raw::Value;
use crate::values::FromJsValue;
use crate::values::JsObjectValue;
use crate::values::JsValue;

pub struct JsString {
    value: Value,
    scope: Scope,
}

impl JsString {
    pub fn new(
        env: &Env,
        text: impl AsRef<str>,
    ) -> crate::Result<Self> {
        let scope = env.scope.current();
        let Some(string) = v8::String::new(scope, text.as_ref()) else {
            return Err(anyhow::anyhow!("Unable to create string"));
        };
        Ok(Self {
            value: Value::from(string.cast()),
            scope: env.scope.clone(),
        })
    }
}

impl JsString {
    pub fn clone(
        &self,
        env: &Env,
    ) -> Self {
        Self {
            value: self.value.clone(),
            scope: env.scope.clone(),
        }
    }
}

impl FromJsValue for JsString {
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

impl JsValue for JsString {
    fn value(&self) -> Value {
        self.value.clone()
    }

    fn scope(&self) -> Scope {
        self.scope.clone()
    }
}

impl JsObjectValue for JsString {}

impl TryFrom<JsString> for String {
    type Error = crate::Error;

    fn try_from(value: JsString) -> Result<Self, Self::Error> {
        todo!()
    }
}
