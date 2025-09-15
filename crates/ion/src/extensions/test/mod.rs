use crate::Env;
use crate::JsExtension;
use crate::JsFunction;
use crate::JsObject;
use crate::JsObjectValue;
use crate::JsString;

static MODULE_NAME: &str = "ion:test";
static BINDING: &str = include_str!("./binding.ts");

fn extension_hook(env: &Env, exports: &mut JsObject) -> crate::Result<()> {
    exports.set_named_property(
        "test",
        JsFunction::new(env, |env, ctx| {
            let message = ctx.arg::<JsString>(0)?;
            let _callback = ctx.arg::<JsFunction>(1)?;


            println!("{}", message.get_string()?);
            Ok(())
        })?,
    )?;

    Ok(())
}

pub fn test() -> JsExtension {
    JsExtension::NativeModuleWithBinding {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
        extension: Box::new(extension_hook),
    }
}
