use ion::*;

pub fn main() -> anyhow::Result<()> {
    // Start the runtime
    let runtime = JsRuntime::initialize_once()?;

    // Register extension with glue code
    runtime.register_extension(JsExtension::NativeModuleWithBinding {
        module_name: "ion:foo".to_string(),
        binding: r#"
            export function foo() {
                return import.meta.extension.foo
            }
        "#
        .to_string(),
        hook: Box::new(|env, exports| {
            let key = env.create_string("foo")?;
            let value = env.create_string("bar")?;
            exports.set_property(key, value)?;
            Ok(())
        }),
    })?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(|env| {
        let result = env.eval_script::<JsString>("globalThis.foo()")?;
        println!("Got: {}", result.get_string()?);
        Ok(())
    })?;

    Ok(())
}
