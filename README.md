# Ion.js ⚡

## JavaScript Runtime for Rust Embedders

Goals:
- ✅ Easy to use high-level API (Inspired by napi-rs)
- ✅ Event-loop built on top of Tokio
- ✅ Simple API to add a standard library
- ✅ Positively multi-threaded
- 👀 C FFI for embedders coming from other languages

*Note: There is still much to do, but this is starting point!*

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
    let runtime = JsRuntime::initialize_debug()?;

    // Create an isolate running on a dedicated thread
    let worker = runtime.spawn_worker()?;

    // // Open a JavaScript context on the isolate thread to execute JavaScript on
    // // You can open multiple contexts, sharing the same thread
    let ctx = worker.create_context()?;

    // Execute some JavaScript in the context
    ctx.exec_blocking(|env| {
        // Evaluate arbitrary JavaScript, the result of the last line is returned
        let value = env.eval_script::<JsNumber>("1 + 1")?;

        // Cast to Rust type
        let result = value.get_u32()?;

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

    // Create an isolate running on a dedicated thread
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