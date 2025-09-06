use std::collections::HashMap;
use std::path::PathBuf;

use crate::ResolverContext;
use crate::platform::JsRealm;
use crate::platform::resolve::run_resolvers;
use crate::utils::PathExt;

pub type ModuleMap = HashMap<i32, PathBuf>;

pub fn run_module(
    realm: &Box<JsRealm>,
    resource_name: String,
    source_string: String,
) {
    let env = realm.env();
    let scope = &mut env.scope();

    let v8_resource_name = v8::String::new(scope, &resource_name).unwrap();
    let source_string = v8::String::new(scope, &source_string).unwrap();

    let origin = v8::ScriptOrigin::new(
        scope,
        v8_resource_name.into(),
        0,
        0,
        false,
        0,
        None,
        false,
        false,
        true,
        None,
    );

    let mut source = v8::script_compiler::Source::new(source_string, Some(&origin));
    let program = v8::script_compiler::compile_module(scope, &mut source).unwrap();
    realm.module_map().insert(
        program.get_identity_hash().into(),
        PathBuf::from(resource_name),
    );

    program
        .instantiate_module(scope, trampoline_instantiate_module)
        .unwrap();

    let promise = program.evaluate(scope).unwrap().cast::<v8::Promise>();
    scope.perform_microtask_checkpoint();
    promise.result(scope);
}

fn trampoline_instantiate_module<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _import_attributes: v8::Local<'a, v8::FixedArray>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };
    let realm = JsRealm::v8_revive(scope);

    let parent_path = realm
        .module_map()
        .get(&referrer.get_identity_hash().into())
        .expect("Could not find parent path");

    let result = realm
        .async_background({
            let ctx = ResolverContext {
                fs: realm.fs.clone(),
                specifier: specifier.to_rust_string_lossy(scope),
                from: parent_path.clone(),
            };
            let resolvers = realm.resolvers.clone();
            async move { run_resolvers(&resolvers, ctx).await }
        })
        .unwrap();

    let code = String::from_utf8(result.code).unwrap();

    {
        let env = realm.env();
        let scope = &mut env.scope();

        let resource_name = v8::String::new(scope, &result.path.try_to_string().unwrap()).unwrap();
        let source_string = v8::String::new(scope, &code).unwrap();

        let origin = v8::ScriptOrigin::new(
            scope,
            resource_name.into(),
            0,
            0,
            false,
            0,
            None,
            false,
            false,
            true,
            None,
        );

        let mut source = v8::script_compiler::Source::new(source_string, Some(&origin));
        let program = v8::script_compiler::compile_module(scope, &mut source).unwrap();

        realm
            .module_map()
            .insert(program.get_identity_hash().into(), result.path);

        Some(program)
    }
}
