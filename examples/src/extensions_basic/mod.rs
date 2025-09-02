use ion::*;

pub fn main() -> anyhow::Result<()> {
    // Start the runtime
    let runtime = ion::platform::initialize_once()?;

    runtime.register_extension(JsExtension::NativeModuleWithBinding {
        module_name: "ion:foo".to_string(),
        binding: r#"
            export function foo() {
                return import.meta.extension.foo
            }
        "#
        .to_string(),
        hook: Box::new(|env, exports| {
            let scope = env.context_scope();
            let global_this = env.global_this();

            let key = v8::String::new(scope, "foo").unwrap();
            let value = v8::String::new(scope, "bar").unwrap();

            global_this.set(scope, key.into(), value.into());
            exports.set(scope, key.into(), value.into());
            Ok(())
        }),
    })?;

    let worker = runtime.spawn_worker()?;

    {
        let ctx = worker.create_context()?;

        ctx.exec_blocking(|env| {
            ion::exts::define_console(&env);
            Ok(())
        })?;

        ctx.exec_blocking(|env| {
            env.eval_script("console.log(globalThis.foo)")?;
            println!("Done");
            Ok(())
        })?;
    };

    Ok(())
}
