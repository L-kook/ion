use crate::JsExtension;

static BINDING: &str = include_str!("./binding.ts");

pub fn global_this() -> JsExtension {
    JsExtension::GlobalBinding {
        binding: BINDING.to_string(),
    }
}
