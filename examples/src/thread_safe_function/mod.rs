use std::thread;
use std::time::Duration;

use ion::JsFunction;
use ion::JsNumber;
use ion::JsRuntime;
use ion::ThreadSafeFunction;

pub fn main() -> anyhow::Result<()> {
    let rt = JsRuntime::initialize_once()?;
    let wrk = rt.spawn_worker()?;
    let ctx = wrk.create_context()?;

    ctx.exec_blocking(|env| {
        let func = JsFunction::new(env, |_env, ctx| {
            let arg0 = ctx.arg::<JsNumber>(0)?;
            println!("tsfn called {}", arg0.get_u32()?);
            Ok(arg0)
        })?;

        let tsfn = ThreadSafeFunction::new(&func)?;

        thread::spawn({
            let tsfn = tsfn.clone();
            move || {
                let ret = tsfn
                    .call_blocking(|_env| Ok(1), |_env, ret| ret.cast::<JsNumber>()?.get_u32())
                    .unwrap();
                println!("Ret: {}", ret);
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn({
            let tsfn = tsfn.clone();
            move || {
                thread::sleep(Duration::from_secs(1));
                let ret = tsfn
                    .call_blocking(|_env| Ok(2), |_env, ret| ret.cast::<JsNumber>()?.get_u32())
                    .unwrap();
                println!("Ret: {}", ret);
                thread::sleep(Duration::from_secs(1));
            }
        });

        Ok(())
    })?;

    Ok(())
}
