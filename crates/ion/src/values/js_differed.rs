use crate::Env;
use crate::ToJsUnknown;
use crate::platform::Value;
use crate::values::FromJsValue;
use crate::values::JsValue;
use crate::values::ToJsValue;

/// JsDiffered is a type that allows for waiting on asynchronous
/// behavior, returning a Promise that can be externally resolved.
///
/// You can think of this as essentially a oneshot channel that
/// returns a Promise to JavaScript
#[derive(Clone)]
pub struct JsDiffered {
    pub(crate) value: Value,
    pub(crate) env: Env,
}

impl JsDiffered {
    pub unsafe fn cast_unchecked<T: FromJsValue>(self) -> T {
        T::from_js_value(&self.env, self.value).expect("Failed to cast JsDiffered")
    }

    pub fn cast<T: FromJsValue>(self) -> crate::Result<T> {
        T::from_js_value(&self.env, self.value)
    }
}

impl JsValue for JsDiffered {
    fn value(&self) -> &Value {
        &self.value
    }

    fn env(&self) -> &Env {
        &self.env
    }
}

impl ToJsUnknown for JsDiffered {}

impl FromJsValue for JsDiffered {
    fn from_js_value(
        env: &Env,
        value: Value,
    ) -> crate::Result<Self> {
        Ok(Self {
            value,
            env: env.clone(),
        })
    }
}

impl ToJsValue for JsDiffered {
    fn to_js_value(
        _env: &Env,
        val: Self,
    ) -> crate::Result<Value> {
        Ok(val.value.clone())
    }
}
