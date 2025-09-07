use std::cell::RefCell;
use std::rc::Rc;

use flume::unbounded;

use crate::Env;
use crate::FromJsValue;
use crate::JsExtension;
use crate::JsFunction;
use crate::JsNumber;
use crate::JsObject;
use crate::JsObjectValue;
use crate::JsUnknown;
use crate::JsValue;
use crate::platform::Value;

static MODULE_NAME: &str = "ion:timers";
static BINDING: &str = include_str!("./binding.js");

enum SetTimeoutEvent {
    RunCallback { timer_ref: u32 },
    Shutdown,
}

fn extension_hook(
    env: &Env,
    exports: &mut JsObject,
) -> crate::Result<()> {
    let run_timeout_callback: Rc<RefCell<Option<v8::Global<v8::Value>>>> = Default::default();
    let (tx, rx) = unbounded::<SetTimeoutEvent>();

    env.on_before_exit({
        let tx = tx.clone();
        move || Ok(tx.try_send(SetTimeoutEvent::Shutdown)?)
    });

    env.spawn_local({
        let env = env.clone();
        let run_timeout_callback = Rc::clone(&run_timeout_callback);
        async move {
            while let Ok(event) = rx.recv_async().await {
                match event {
                    SetTimeoutEvent::RunCallback { timer_ref } => {
                        let scope = &mut env.scope();

                        let cell = run_timeout_callback.borrow();
                        let callback = cell.as_ref().unwrap();
                        let callback = v8::Local::new(scope, callback);
                        let callback = JsFunction::from_js_value(&env, Value::from(callback))?;

                        let timer_ref = env.create_uint32(timer_ref)?;
                        callback.call_with_args::<JsUnknown, _>(timer_ref)?;
                    }
                    SetTimeoutEvent::Shutdown => break,
                }
            }
            Ok(())
        }
    })?;

    exports.set_named_property(
        "onTimeoutCallback",
        JsFunction::new(env, {
            let run_timeout_callback = Rc::clone(&run_timeout_callback);
            move |env, ctx| {
                let scope = &mut env.scope();

                let arg0 = ctx.arg::<JsFunction>(0)?;
                let arg0 = v8::Global::new(scope, arg0.value().inner());

                let mut cell = run_timeout_callback.borrow_mut();
                cell.replace(arg0);
                Ok(())
            }
        })?,
    )?;

    exports.set_named_property(
        "createSetTimeout",
        JsFunction::new(env, {
            move |env, ctx| {
                let timer_ref = ctx.arg::<JsNumber>(0)?.get_u32()?;
                let duration = ctx.arg::<JsNumber>(1)?.get_u32()?;

                env.spawn_background({
                    let tx = tx.clone();
                    async move {
                        tokio::time::sleep(tokio::time::Duration::from_millis(duration as u64))
                            .await;

                        tx.try_send(SetTimeoutEvent::RunCallback { timer_ref })?;
                        Ok(())
                    }
                })?;

                Ok(())
            }
        })?,
    )?;

    Ok(())
}

pub fn set_timeout() -> JsExtension {
    JsExtension::NativeModuleWithBinding {
        module_name: MODULE_NAME.to_string(),
        binding: BINDING.to_string(),
        extension: Box::new(extension_hook),
    }
}
