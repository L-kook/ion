use std::sync::Arc;

use flume::RecvError;
use flume::TrySendError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    IO(Arc<std::io::Error>),
    Generic(String),
    PlatformCommunicationError,
    PlatformInitializeError,
    IsolateNotInitializedError,
    EventLoopNotInitializedError,
    WorkerInitializeError,
    ValueCreateError,
    ValueGetError,
    ValueCastError,
    ScriptCompileError,
    ScriptRunError,
    ExecError,
    TaskSpawnError,
    OutOfBounds,
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(Arc::new(value))
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_value: TrySendError<T>) -> Self {
        Error::PlatformCommunicationError
    }
}

impl From<RecvError> for Error {
    fn from(_value: RecvError) -> Self {
        Error::PlatformCommunicationError
    }
}
