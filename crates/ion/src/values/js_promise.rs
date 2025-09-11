use parking_lot::Mutex;

// TODO
use crate::Env;
use crate::JsFunction;
use crate::JsUnknown;
use crate::ToJsUnknown;
use crate::platform::sys;
use crate::platform::sys::Value;
use crate::utils::v8::v8_create_string;
use crate::values::FromJsValue;
use crate::values::JsValue;
use crate::values::ToJsValue;

#[derive(Clone)]
pub struct JsPromise {
    pub(crate) value: Value,
    pub(crate) env: Env,
}

impl JsPromise {
    pub fn new(env: &Env) -> crate::Result<JsPromise> {
        let scope = &mut env.scope();
        let object = v8::Object::new(scope);
        Ok(Self {
            value: sys::v8_from_value(object),
            env: env.clone(),
        })
    }

    /// Non blocking call to then/catch
    pub fn settled<Resolved: FromJsValue + 'static>(
        self,
        settled_callback: impl 'static
        + Send
        + Sync
        + FnOnce(&Env, JsPromiseResult<Resolved>) -> crate::Result<()>,
    ) -> crate::Result<()> {
        #[allow(clippy::type_complexity)]
        let settled_callback: Mutex<
            Option<
                Box<
                    dyn 'static
                        + Send
                        + Sync
                        + FnOnce(&Env, JsPromiseResult<Resolved>) -> crate::Result<()>,
                >,
            >,
        > = Mutex::new(Some(Box::new(settled_callback)));

        let scope = &mut self.env.scope();

        let promise = sys::v8_value_cast::<v8::Object, _>(self.value);
        let then_key = v8_create_string(scope, "then")?;
        let Some(then) = promise.get(scope, sys::v8_value_cast::<v8::Value, _>(then_key)) else {
            return Err(crate::Error::ValueGetError);
        };
        let then_fn = sys::v8_value_cast::<v8::Function, _>(then);

        let then_fn_recv = JsFunction::new(&self.env, move |env, ctx| {
            let settled_callback = {
                let mut settled_callback = settled_callback.lock();
                settled_callback.take().unwrap()
            };
            let result = ctx.arg::<JsUnknown>(0)?;
            let result = Resolved::from_js_value(env, *result.value())?;
            let result = JsPromiseResult::<Resolved>::Resolved(result);
            settled_callback(env, result)
        })?;

        then_fn.call(scope, promise.into(), &[then_fn_recv.value]);

        Ok(())
    }
}

pub enum JsPromiseResult<Result: FromJsValue> {
    Resolved(Result),
    Rejected(JsUnknown),
}

impl JsValue for JsPromise {
    fn value(&self) -> &Value {
        &self.value
    }

    fn env(&self) -> &Env {
        &self.env
    }
}

impl ToJsUnknown for JsPromise {}

impl FromJsValue for JsPromise {
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

impl ToJsValue for JsPromise {
    fn to_js_value(
        _env: &Env,
        val: Self,
    ) -> crate::Result<Value> {
        Ok(val.value)
    }
}
