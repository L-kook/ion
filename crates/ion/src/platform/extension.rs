use std::sync::Arc;

use crate::JsExtension;
use crate::JsObject;
use crate::JsValue;
use crate::platform::JsRealm;
use crate::platform::module::Module;
use crate::platform::module::ModuleStatus;
use crate::platform::module::init_meta_callback;

pub struct Extension {}

impl Extension {
    pub fn register_extensions(
        realm: &JsRealm,
        extensions: &Vec<Arc<JsExtension>>,
    ) -> crate::Result<()> {
        for extension in extensions {
            match extension.as_ref() {
                JsExtension::NativeModuleWithBinding {
                    module_name,
                    binding,
                    extension,
                } => {
                    let env = realm.env();
                    let module_map = realm.module_map();

                    // Run extension hook
                    let mut exports = JsObject::new(&env)?;
                    extension(&env, &mut exports)?;

                    // Construct module for binding
                    let module = Module::new(realm, module_name, binding)?;
                    let v8_module = module.v8_module();
                    module_map.insert(module);

                    // Initialize binding module
                    let scope = &mut env.scope();

                    // TEMP, use data or statics or something
                    {
                        let global_this = env.global_this()?;
                        let global_this = global_this.value().inner().cast::<v8::Object>();
                        let key = v8::Integer::new(scope, v8_module.get_identity_hash().into());
                        let value = exports.value().inner();
                        global_this.set(scope, key.into(), value.into());
                    };

                    // Initialize extension module
                    scope.set_host_initialize_import_meta_object_callback(init_meta_callback);

                    v8_module
                        .instantiate_module(scope, Module::v8_initialize_callback)
                        .unwrap();

                    let promise = v8_module.evaluate(scope).unwrap().cast::<v8::Promise>();
                    scope.perform_microtask_checkpoint();
                    promise.result(scope);

                    let module = module_map.get_module_mut(module_name).unwrap();
                    module.status = ModuleStatus::Ready;
                }
                JsExtension::NativeModule {
                    module_name: _,
                    extension: _,
                } => {
                    // CreateSyntheticModule
                }
                JsExtension::NativeGlobal { hook: _ } => {
                    // Exec
                }
                JsExtension::GlobalBinding { binding: _ } => {
                    // Eval
                }
            }
        }

        Ok(())
    }
}

/*
    // DEBUG
    {
        let env = realm.env();
        let scope = &mut env.scope();
        let module_map = realm.module_map();

        dbg!(&module_map);

        let module = module_map.get_module(module_name).unwrap();
        let v8_module = module.v8_module();

        let exports = v8_module.get_module_namespace().cast::<v8::Object>();

        let exports_arr = exports
            .get_property_names(scope, Default::default())
            .unwrap();

        println!("len {}", exports_arr.length());
        for i in 0..exports_arr.length() {
            let i = v8::Number::new(scope, i as _).into();
            let key = exports_arr.get(scope, i).unwrap();
            println!(
                "exports: {:?} -> {:?}",
                key.to_rust_string_lossy(scope),
                exports.get(scope, key.into()).unwrap().cast::<v8::Value>(),
                // .to_rust_string_lossy(scope),
            );
        }
    };

*/
