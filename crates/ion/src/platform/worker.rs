#![allow(warnings)]
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::usize;

use flume::Receiver;
use flume::Sender;
use flume::bounded;
use flume::unbounded;

use crate::Env;
use crate::Error;
use crate::JsExtension;
use crate::utils::channel::oneshot;
use crate::utils::tokio_ext::LocalRuntimeExt;

pub(crate) enum JsWorkerEvent {
    CreateContext(Sender<(usize, Sender<JsWorkerEvent>)>),
    ShutdownContext(usize, Sender<()>),
    Exec(usize, Box<dyn Send + FnOnce(Env) -> crate::Result<()>>),
    Shutdown(Sender<()>),
}

// Create a dedicated thread to host the isolate
pub(crate) fn start_js_worker_thread() -> (Sender<JsWorkerEvent>, Mutex<Option<JoinHandle<()>>>) {
    let (tx, rx) = unbounded::<JsWorkerEvent>();

    let handle = thread::spawn({
        let tx: Sender<JsWorkerEvent> = tx.clone();
        move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .local_block_on(worker_thread_async(tx, rx));
        }
    });

    (tx, Mutex::new(Some(handle)))
}

async fn worker_thread_async(
    tx: Sender<JsWorkerEvent>,
    rx: Receiver<JsWorkerEvent>,
) {
    // Maintain a store of contexts to help with cleanup on shutdown.
    let mut contexts = HashMap::<usize, Env>::new();

    // Create an isolate dedicated to this "worker" thread
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let isolate_ptr = isolate.as_mut() as *mut v8::Isolate;

    while let Ok(event) = rx.recv_async().await {
        match event {
            JsWorkerEvent::CreateContext(sender) => todo!(),
            JsWorkerEvent::ShutdownContext(_, sender) => todo!(),
            JsWorkerEvent::Exec(_, fn_once) => todo!(),
            JsWorkerEvent::Shutdown(sender) => todo!(),
        }
    }
}
