use crate::Env;

pub type ExtensionHook =
    Box<dyn 'static + Sync + Send + Fn(&Env, &mut v8::Local<'_, v8::Object>) -> crate::Result<()>>;

pub enum JsExtension {
    /// Extension available as a module that has both native code and an associated JavaScript glue code binding
    NativeModuleWithBinding {
        module_name: String,
        binding: String,
        hook: ExtensionHook,
    },
    /// Extension available as a module that is written in native code
    NativeModule {
        module_name: String,
        hook: ExtensionHook,
    },
    /// Extension that runs native code when a JsContext is started, used to mutate globalThis
    NativeGlobal { hook: ExtensionHook },
    /// Extension that runs JavaScript code when a JsContext is started, used to mutate globalThis
    GlobalBinding { binding: String },
}

impl std::fmt::Debug for JsExtension {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            JsExtension::NativeModuleWithBinding {
                module_name,
                binding: _,
                hook: _,
            } => write!(f, "NativeModuleWithBinding({})", module_name),
            JsExtension::NativeModule {
                module_name,
                hook: _,
            } => {
                write!(f, "NativeModule({})", module_name)
            }
            JsExtension::NativeGlobal { hook: _ } => {
                write!(f, "NativeGlobal(unnamed)")
            }
            JsExtension::GlobalBinding { binding: _ } => {
                write!(f, "GlobalBinding(unnamed)")
            }
        }
    }
}
