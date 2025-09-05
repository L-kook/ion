use std::sync::Arc;
use std::sync::atomic::Ordering;

use flume::Sender;
use flume::bounded;

use crate::Error;
use crate::JsExtension;
use crate::JsWorker;
use crate::platform::platform::HAS_INIT;
use crate::platform::platform::PLATFORM;
use crate::utils::channel::oneshot;

use super::platform::platform::PlatformEvent;

/// JsRuntime is a handle to the underlying v8 engine. It can spawn worker threads and
/// evaluate JavaScript within Contexts
#[derive(Clone, Debug)]
pub struct JsRuntime {
    pub(crate) tx: Sender<crate::platform::platform::PlatformEvent>,
}

impl JsRuntime {
    /// Initialize the v8 runtime, this can only be done once per process.
    /// Subsequent calls will return the first instance of [`JsRuntime`]
    pub fn initialize_once_with_args(args: &[&str]) -> crate::Result<Arc<JsRuntime>> {
        let args = args.iter().map(|v| v.to_string()).collect::<Vec<String>>();

        if PLATFORM.send(PlatformEvent::Init { args }).is_err() {
            return Err(crate::Error::PlatformInitializeError);
        };

        Ok(Arc::new(JsRuntime {
            tx: PLATFORM.clone(),
        }))
    }

    /// Initialize the v8 runtime, this can only be done once per process.
    /// Subsequent calls will return the first instance of [`JsRuntime`]
    pub fn initialize_once() -> crate::Result<Arc<JsRuntime>> {
        Self::initialize_once_with_args(&[])
    }

    /// Check if the v8 runtime has already been initialized
    pub fn has_initialized() -> bool {
        HAS_INIT.load(Ordering::Acquire)
    }

    /// Spawns a dedicated worker thread for isolates
    pub fn spawn_worker(&self) -> crate::Result<Arc<JsWorker>> {
        let (tx, rx) = bounded(1);

        if self
            .tx
            .send(PlatformEvent::SpawnWorker { resolve: tx })
            .is_err()
        {
            return Err(Error::WorkerInitializeError);
        };

        let Ok((tx, handle)) = rx.recv() else {
            return Err(Error::WorkerInitializeError);
        };

        Ok(Arc::new(JsWorker::new(tx, handle)))
    }

    /// Register a native extension, available in all contexts
    pub fn register_extension(
        &self,
        extension: JsExtension,
    ) -> crate::Result<()> {
        let (tx, rx) = oneshot();
        self.tx
            .try_send(PlatformEvent::RegisterExtension(extension, tx))?;
        rx.recv()?
    }
}

impl Drop for JsRuntime {
    fn drop(&mut self) {
        // Once initialized, JsRuntime is never dropped
        // Only the worker threads and isolates are dropped
    }
}
