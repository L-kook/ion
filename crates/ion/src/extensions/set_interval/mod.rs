use crate::Env;
use crate::JsExtension;
use crate::JsObject;

static MODULE_NAME: &str = "ion:timers/interval";
static BINDING: &str = include_str!("./binding.js");

fn extension_hook(
    _env: &Env,
    _exports: &mut JsObject,
) -> crate::Result<()> {
    Ok(())
}

pub fn set_interval() -> JsExtension {
    JsExtension::NativeModuleWithBinding {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
        extension: Box::new(extension_hook),
    }
}
