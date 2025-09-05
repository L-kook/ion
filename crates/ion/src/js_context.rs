use flume::Sender;
use flume::bounded;

use crate::Env;
use crate::Error;
use crate::platform::worker::JsWorkerEvent;
use crate::utils::channel::oneshot;

/// This is a handle to a v8::Context
#[derive(Debug, Clone)]
pub struct JsContext {
    pub(crate) id: usize,
    pub(crate) tx: Sender<JsWorkerEvent>,
}

impl JsContext {
    pub fn exec(
        &self,
        callback: impl 'static + Send + FnOnce(Env) -> crate::Result<()>,
    ) -> crate::Result<()> {
        if self
            .tx
            .try_send(JsWorkerEvent::Exec(self.id, Box::new(callback)))
            .is_err()
        {
            return Err(Error::ExecError);
        };
        Ok(())
    }

    pub async fn exec_async(
        &self,
        callback: impl 'static + Send + FnOnce(Env) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let (tx, rx) = bounded(1);

        self.exec(move |env| {
            callback(env)?;
            Ok(tx.try_send(())?)
        })?;

        if rx.recv_async().await.is_err() {
            return Err(Error::ExecError);
        };

        Ok(())
    }

    pub fn exec_blocking(
        &self,
        callback: impl 'static + Send + FnOnce(Env) -> crate::Result<()>,
    ) -> crate::Result<()> {
        let (tx, rx) = bounded(1);

        self.exec(move |env| {
            callback(env)?;
            Ok(tx.try_send(())?)
        })?;

        if rx.recv().is_err() {
            return Err(Error::ExecError);
        };
        Ok(())
    }
}

impl Drop for JsContext {
    fn drop(&mut self) {
        let (tx, rx) = oneshot();

        if self
            .tx
            .send(JsWorkerEvent::ShutdownContext(self.id, tx))
            .is_err()
        {
            panic!("Cannot drop JsContext")
        };

        if rx.recv().is_err() {
            panic!("Cannot drop JsContext")
        }
    }
}
