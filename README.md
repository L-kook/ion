# Ion.js ⚡

## A JavaScript Runtime for Rust

Ion is a JavaScript runtime for integrating a JavaScript engine within a Rust program. This is useful for cases like building out a JavaScript powered plug-in system - but it can also be used directly from a stand alone executable.

Goals:
- ✅ Easy to use high-level API (Inspired by napi-rs)
- ✅ Event-loop built on top of Tokio
- ✅ Simple API to add a standard library
- ✅ Positively multi-threaded
- 👀 C FFI for embedders coming from other languages



## CLI Usage

The repo includes a reference executable that implements Ion, you can find it under [./crates/ion_cli](./crates/ion_cli)

```bash
cargo build --release
./target/release/ion_cli eval "console.log('42')"
```

## Embedder Usage

### Basic

For more, see [./examples](./examples)

```rust
use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    // Create an isolate running on a dedicated thread
    let worker = runtime.spawn_worker()?;

    // Open a JavaScript context (a fresh globalThis) to execute JavaScript.
    // You can open multiple contexts, sharing the same thread
    let ctx = worker.create_context()?;

    // Execute some JavaScript in the context
    ctx.exec_blocking(|env| {
        // Evaluate arbitrary JavaScript, the result of the last line is returned
        let value = env.eval_script::<JsNumber>("1 + 1")?;

        // Cast to Rust type
        let result = value.get_u32()?;

        // Prints "2"
        println!("Returned: {}", result);
        Ok(())
    })?;

    Ok(())
}

```

### Async

```rust
use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    ctx.exec_blocking(|env| {
        // Spawn an future on the event loop
        env.spawn_local({
            let env = env.clone();
            async move {
                println!("Async Task Started");

                let value = env.eval_script::<JsNumber>("1 + 1")?;

                // Wait for some time
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

                println!("Async Task Returned: {}", value.get_u32()?);

                Ok(())
            }
        })?;

        Ok(())
    })?;

    Ok(())
}
```

### Calling JavaScript from another Rust thread

```rust
use ion::*;

pub fn main() -> anyhow::Result<()> {
    let runtime = JsRuntime::initialize_once()?;

    let worker = runtime.spawn_worker()?;
    let ctx = worker.create_context()?;

    // Create a function on the global scope
    ctx.eval_script("globalThis.add = (a, b) => a + b")

    // Execute some Rust code within the JavaScript realm
    ctx.exec_blocking(|env| {
        let global_this = env.global_this()?;
        let function = global_this.get_named_property_unchecked::<JsFunction>("foo")?;

        // Create a reference counted thread safe handle to the function
        let tsfn = ThreadSafeFunction::new(&function)?;

        // Send that function into a thread and call it
        thread::spawn(move || {
            let ret: u32 = tsfn
                .call_blocking(
                    // Map Rust values to be used as JavaScript values
                    |env| Ok((1, 1)), 
                    // Map the return type to be used in Rust
                    |env, ret| ret.cast::<JsNumber>()?.get_u32(),
                )
                .unwrap();

            println!("JavaScript function returned: {}", ret); // "3"
            thread::sleep(Duration::from_secs(1));
        });

        Ok(())
    })?;

    Ok(())
}
```
