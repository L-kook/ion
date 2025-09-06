#![allow(unused)]

pub fn bench(callback: impl 'static + Fn() -> anyhow::Result<()>) -> anyhow::Result<()> {
    println!("Bench");
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Running once");
    callback()?;
    std::thread::sleep(std::time::Duration::from_secs(3));

    println!("Looping");
    for i in 0..50 {
        println!("loop {}", i);
        for _ in 0..10000 {
            callback()?;
        }
    }

    println!("done");
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}

pub fn v8_eval_script<'a, S: AsRef<str>>(
    scope: &mut v8::CallbackScope<'a>,
    code: S,
) -> crate::Result<v8::Local<'a, v8::Value>> {
    let Some(code) = v8::String::new(scope, code.as_ref()) else {
        panic!();
    };
    let Some(script) = v8::Script::compile(scope, code, None) else {
        panic!();
    };
    let Some(value) = script.run(scope) else {
        panic!();
    };
    Ok(value)
}

pub fn v8_eval_print(
    scope: &mut v8::CallbackScope<'_>,
    code: impl AsRef<str>,
) {
    let result = v8_eval_script(
        scope,
        format!("JSON.stringify((() => {{ return {} }})())", code.as_ref()),
    )
    .unwrap();
    println!("Result: {}", result.to_rust_string_lossy(scope));
}

pub fn v8_trigger_gc(isolate_ptr: *mut v8::Isolate) {
    (unsafe { &mut *isolate_ptr })
        .request_garbage_collection_for_testing(v8::GarbageCollectionType::Full);
}
