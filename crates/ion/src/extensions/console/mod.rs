use crate::Env;
use crate::JsExtension;
use crate::JsObject;

static MODULE_NAME: &str = "ion:console";
static BINDING: &str = include_str!("./binding.js");

fn extension_hook(
    _env: &Env,
    _exports: &mut JsObject,
) -> crate::Result<()> {
    // TODO
    Ok(())
}

pub fn console() -> JsExtension {
    JsExtension::NativeModuleWithBinding {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
        extension: Box::new(extension_hook),
    }
}
