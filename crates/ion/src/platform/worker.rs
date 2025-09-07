#![allow(warnings)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::usize;

use flume::Receiver;
use flume::Sender;
use flume::bounded;
use flume::unbounded;
use tokio_util::task::TaskTracker;

use crate::DynResolver;
use crate::Env;
use crate::Error;
use crate::JsExtension;
use crate::ResolverContext;
use crate::fs::FileSystem;
use crate::platform::JsRealm;
use crate::platform::background_worker::BackgroundWorkerEvent;
use crate::platform::module::Module;
use crate::platform::module_map::ModuleMap;
use crate::platform::resolve::run_resolvers;
use crate::utils::HashMapExt;
use crate::utils::PathExt;
use crate::utils::channel::oneshot;
use crate::utils::tokio_ext::LocalRuntimeExt;

pub(crate) enum JsWorkerEvent {
    CreateContext {
        resolve: Sender<(usize, Sender<JsWorkerEvent>)>,
    },
    ShutdownContext {
        id: usize,
        resolve: Sender<()>,
    },
    Exec {
        id: usize,
        callback: Box<dyn Send + FnOnce(&Env) -> crate::Result<()>>,
    },
    Import {
        id: usize,
        specifier: String,
        resolve: Sender<()>,
    },
    Shutdown {
        resolve: Sender<()>,
    },
}

// Create a dedicated thread to host the isolate
pub(crate) fn start_js_worker_thread(
    tx_background: Sender<BackgroundWorkerEvent>,
    extensions: Vec<Arc<JsExtension>>,
    resolvers: Vec<DynResolver>,
) -> (Sender<JsWorkerEvent>, Mutex<Option<JoinHandle<()>>>) {
    let (tx, rx) = unbounded::<JsWorkerEvent>();

    let handle = thread::spawn({
        let tx: Sender<JsWorkerEvent> = tx.clone();
        move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .local_block_on(worker_thread_async(
                    tx,
                    rx,
                    tx_background,
                    extensions,
                    resolvers,
                ));
        }
    });

    (tx, Mutex::new(Some(handle)))
}

async fn worker_thread_async(
    tx: Sender<JsWorkerEvent>,
    rx: Receiver<JsWorkerEvent>,
    tx_background: Sender<BackgroundWorkerEvent>,
    extensions: Vec<Arc<JsExtension>>,
    resolvers: Vec<DynResolver>,
) -> crate::Result<()> {
    // Maintain a store of contexts to help with cleanup on shutdown.
    let mut realms = HashMap::<usize, Box<JsRealm>>::new();
    let fs = FileSystem::Physical;

    // Create an isolate dedicated to this "worker" thread
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let isolate_ptr = isolate.as_mut() as *mut v8::Isolate;

    while let Ok(event) = rx.recv_async().await {
        match event {
            JsWorkerEvent::CreateContext { resolve } => {
                let realm = JsRealm::new(
                    isolate_ptr,
                    fs.clone(),
                    resolvers.clone(),
                    tx_background.clone(),
                );
                let realm_id = realm.id();

                realms.insert(realm_id.clone(), realm);
                resolve.try_send((realm_id, tx.clone()))?;
            }
            JsWorkerEvent::ShutdownContext { id, resolve } => {
                let realm = realms.try_remove(&id)?;
                realm.drain_async_tasks().await;
                resolve.try_send(())?;
            }
            JsWorkerEvent::Exec { id, callback } => {
                let realm = realms.try_get(&id)?;
                if let Err(err) = callback(&realm.env()) {
                    // TODO global error handler
                    panic!("Callback errored {:?}", err)
                };
            }
            JsWorkerEvent::Import {
                id,
                specifier,
                resolve,
            } => {
                Module::v8_initialize(
                    true,
                    realms.try_get(&id)?,
                    &specifier,
                    std::env::current_dir()?.try_to_string()?,
                );

                // {
                //     let realm = realms.try_get(&id)?;
                //     let env = realm.env();
                //     let scope = &mut env.scope();
                //     let module_map = realm.module_map();

                //     dbg!(&module_map);

                //     let module = module_map.get_module(&specifier).unwrap();
                //     let v8_module = module.v8_module();

                //     let exports = v8_module.get_module_namespace().cast::<v8::Object>();

                //     let exports_arr = exports
                //         .get_property_names(scope, Default::default())
                //         .unwrap();

                //     println!("len {}", exports_arr.length());
                //     for i in 0..exports_arr.length() {
                //         let i = v8::Number::new(scope, i as _).into();
                //         let key = exports_arr.get(scope, i).unwrap();
                //         println!(
                //             "exports: {:?} -> {:?}",
                //             key.to_rust_string_lossy(scope),
                //             exports
                //                 .get(scope, key.into())
                //                 .unwrap()
                //                 .cast::<v8::String>()
                //                 .to_rust_string_lossy(scope),
                //         );
                //     }
                // };

                let result = resolve.try_send(())?;
            }
            JsWorkerEvent::Shutdown { resolve } => {
                for (_id, realm) in realms {
                    realm.drain_async_tasks().await;
                }
                resolve.try_send(())?;
                break;
            }
        }
    }

    Ok(())
}
