use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

use flume::Sender;
use flume::bounded;

use crate::Error;
use crate::platform::worker::JsWorkerEvent;
use crate::utils::channel::oneshot;

use super::JsContext;

/// This is a handle to a v8::Isolate running on a dedicated thread.
/// A worker thread can spawn multiple v8::Contexts within that thread
/// to be used to execute JavaScript
#[derive(Debug)]
pub struct JsWorker {
    tx: Sender<JsWorkerEvent>,
    handle: Mutex<Option<JoinHandle<crate::Result<()>>>>,
}

impl JsWorker {
    pub(crate) fn new(
        tx: Sender<JsWorkerEvent>,
        handle: Mutex<Option<JoinHandle<crate::Result<()>>>>,
    ) -> Self {
        JsWorker { tx, handle }
    }

    /// Create a handle to a v8::Context associated with this v8::Isolate
    pub fn create_context(&self) -> crate::Result<Arc<JsContext>> {
        let (tx, rx) = bounded(1);

        if self
            .tx
            .send(JsWorkerEvent::CreateContext { resolve: tx })
            .is_err()
        {
            return Err(Error::WorkerInitializeError);
        };

        let Ok((id, tx)) = rx.recv() else {
            return Err(Error::WorkerInitializeError);
        };

        Ok(Arc::new(JsContext { id, tx }))
    }

    pub fn run_garbage_collection_for_testing(&self) -> crate::Result<()> {
        let (tx, rx) = bounded(1);

        if self
            .tx
            .send(JsWorkerEvent::RunGarbageCollectionForTesting { resolve: tx })
            .is_err()
        {
            return Err(Error::WorkerInitializeError);
        };

        Ok(rx.recv()?)
    }
}

impl Drop for JsWorker {
    fn drop(&mut self) {
        let (tx, rx) = oneshot();

        if self
            .tx
            .send(JsWorkerEvent::RequestShutdown { resolve: tx })
            .is_err()
        {
            panic!("Cannot drop JsWorker 1");
        };

        if rx.recv().is_err() {
            panic!("Cannot drop JsWorker 2");
        }

        let Ok(mut handle) = self.handle.lock() else {
            panic!("Cannot drop JsWorker 3");
        };

        if let Some(handle) = handle.take() {
            drop(handle.join().unwrap());
        }
    }
}
