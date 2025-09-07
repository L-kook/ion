use crate::JsExtension;

static MODULE_NAME: &str = "ion:timers";
static BINDING: &str = include_str!("./binding.js");

pub fn timers() -> JsExtension {
    JsExtension::BindingModule {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
    }
}
