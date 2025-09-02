use crate::env::Env;
use crate::raw::Scope;
use crate::raw::Value;

// #[derive(Debug, Clone)]
// pub struct RawValue {
//     pub (crate) value: Value,
//     pub (crate) scope: ScopeRef,
// }

pub trait FromJsValue: Sized {
    /// this function called to convert JavaScript values to native rust values
    fn from_js_value(
        env: &Env,
        value: Value,
    ) -> crate::Result<Self>;
}

pub trait JsValue: Sized + FromJsValue {
    fn value(&self) -> Value;
    fn scope(&self) -> Scope;
}

pub trait JsObjectValue: JsValue {
    /// Set the property value to the `Object`
    fn set_property<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> crate::Result<()>
    where
        K: JsValue,
        V: JsValue,
    {
        let scope = self.scope();

        let object_value = self.value();
        let object_raw = object_value.inner();
        let object = object_raw.cast::<v8::Object>();

        let key_value = key.value();
        let key = key_value.inner();

        let value_value = value.value();
        let value = value_value.inner();

        object.set(scope.current(), key, value);
        Ok(())
    }
}
