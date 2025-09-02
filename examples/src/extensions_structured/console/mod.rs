use ion::Env;
use ion::JsExtension;

static MODULE_NAME: &str = "ion:console";
static BINDING: &str = include_str!("./binding.js");

fn extension_hook(
    _env: &Env,
    _exports: &mut v8::Local<'_, v8::Object>,
) -> ion::Result<()> {
    // TODO
    Ok(())
}

pub fn extension() -> JsExtension {
    JsExtension::NativeModuleWithBinding {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
        hook: Box::new(extension_hook),
    }
}
