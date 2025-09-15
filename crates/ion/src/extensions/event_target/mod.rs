use crate::JsExtension;

static MODULE_NAME: &str = "ion:event_target";
static BINDING: &str = include_str!("./binding.ts");

pub fn event_target() -> JsExtension {
    JsExtension::BindingModule {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
    }
}
