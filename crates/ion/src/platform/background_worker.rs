use std::pin::Pin;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::thread::{self};

use flume::Receiver;
use flume::Sender;
use flume::unbounded;

pub(crate) enum BackgroundWorkerEvent {
    ExecFut(Pin<Box<dyn 'static + Send + Sync + Future<Output = crate::Result<()>>>>),
}

impl std::fmt::Debug for BackgroundWorkerEvent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::ExecFut(_) => write!(f, "ExecFut"),
        }
    }
}

pub(crate) fn start_background_worker_thread()
-> (Sender<BackgroundWorkerEvent>, Mutex<Option<JoinHandle<()>>>) {
    let (tx, rx) = unbounded::<BackgroundWorkerEvent>();

    let handle = thread::spawn({
        move || {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(num_cpus::get_physical())
                .enable_all()
                .build()
                .unwrap()
                .block_on(background_worker_thread_async(rx))
                .unwrap();
        }
    });

    (tx, Mutex::new(Some(handle)))
}

async fn background_worker_thread_async(rx: Receiver<BackgroundWorkerEvent>) -> crate::Result<()> {
    while let Ok(event) = rx.recv_async().await {
        tokio::task::spawn(async move {
            match event {
                BackgroundWorkerEvent::ExecFut(future) => {
                    future.await.unwrap();
                }
            }
        });
    }

    Ok(())
}
