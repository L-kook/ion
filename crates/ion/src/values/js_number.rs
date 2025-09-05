use crate::Env;
use crate::ToJsUnknown;
use crate::platform::Value;
use crate::values::FromJsValue;
use crate::values::JsValue;
use crate::values::ToJsValue;

#[derive(Clone)]
pub struct JsNumber {
    pub(crate) value: Value,
    pub(crate) env: Env,
}

impl JsNumber {
    pub fn from_u32(
        env: &Env,
        val: u32,
    ) -> crate::Result<Self> {
        let scope = &mut env.scope();

        let local = v8::Integer::new_from_unsigned(scope, val);
        let value = Value::from(local.cast::<v8::Value>());
        Ok(Self {
            value,
            env: env.clone(),
        })
    }

    pub fn from_i32(
        env: &Env,
        val: i32,
    ) -> crate::Result<Self> {
        let scope = &mut env.scope();

        let local = v8::Integer::new(scope, val);
        let value = Value::from(local.cast::<v8::Value>());
        Ok(Self {
            value,
            env: env.clone(),
        })
    }

    pub fn from_f64(
        env: &Env,
        val: f64,
    ) -> crate::Result<Self> {
        let scope = &mut env.scope();

        let local = v8::Number::new(scope, val);
        let value = Value::from(local.cast::<v8::Value>());
        Ok(Self {
            value,
            env: env.clone(),
        })
    }

    pub fn get_u32(&self) -> crate::Result<u32> {
        let scope = &mut self.env.scope();

        let local = self.value.inner();
        let local = local.cast::<v8::Integer>();
        let Some(value) = local.uint32_value(scope) else {
            return Err(crate::Error::ValueGetError);
        };
        Ok(value)
    }

    pub fn get_i32(&self) -> crate::Result<i32> {
        let scope = &mut self.env.scope();

        let local = self.value.inner();
        let local = local.cast::<v8::Integer>();
        let Some(value) = local.int32_value(scope) else {
            return Err(crate::Error::ValueGetError);
        };
        Ok(value)
    }

    pub fn get_f64(&self) -> crate::Result<f64> {
        let local = self.value.inner();
        let local = local.cast::<v8::Number>();
        Ok(local.value())
    }
}

impl JsValue for JsNumber {
    fn value(&self) -> &Value {
        &self.value
    }

    fn env(&self) -> &Env {
        &self.env
    }
}

impl ToJsUnknown for JsNumber {}

impl FromJsValue for JsNumber {
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

impl ToJsValue for JsNumber {
    fn to_js_value(
        _env: &Env,
        val: Self,
    ) -> crate::Result<Value> {
        Ok(val.value.clone())
    }
}

impl Env {
    pub fn int32_value(
        &self,
        value: i32,
    ) -> crate::Result<JsNumber> {
        JsNumber::from_i32(self, value)
    }
}
