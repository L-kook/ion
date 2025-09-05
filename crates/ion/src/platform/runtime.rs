use std::sync::Arc;

use flume::Sender;
use flume::bounded;

use crate::JsExtension;
use crate::utils::channel::oneshot;
use crate::Error;

use super::JsWorker;
use super::platform::PlatformEvent;

/// JsRuntime is a handle to the underlying v8 engine. It can spawn worker threads and
/// evaluate JavaScript within Contexts
#[derive(Clone, Debug)]
pub struct JsRuntime {
    pub(crate) tx: Sender<super::platform::PlatformEvent>,
}

impl JsRuntime {
    /// Spawns a dedicated worker thread for isolates
    pub fn spawn_worker(&self) -> crate::Result<Arc<JsWorker>> {
        let (tx, rx) = bounded(1);

        if self.tx.send(PlatformEvent::SpawnWorker(tx)).is_err() {
            return Err(Error::WorkerInitializeError);
        };

        let Ok(worker) = rx.recv() else {
            return Err(Error::WorkerInitializeError);
        };

        Ok(worker)
    }

    /// Register a native extension, available in all contexts
    pub fn register_extension(
        &self,
        extension: JsExtension,
    ) -> crate::Result<()> {
        let (tx, rx) = oneshot();
        self.tx
            .try_send(PlatformEvent::RegisterExtension(extension, tx))
            .unwrap();
        rx.recv().unwrap()
    }
}

impl Drop for JsRuntime {
    fn drop(&mut self) {
        // Once initialized, JsRuntime is never dropped
        // Only the worker threads and isolates are dropped
    }
}
