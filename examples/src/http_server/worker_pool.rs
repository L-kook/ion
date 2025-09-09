use ion::utils::channel::Sender;
use ion::utils::channel::channel;
use ion::utils::channel::oneshot;
use ion::*;
use tokio::task::JoinHandle;

// Basic load balancer, round robin
pub struct WorkerPool {
    queue: Sender<Box<dyn 'static + Send + FnOnce(&Env) -> ion::Result<()>>>,
}

impl WorkerPool {
    pub fn new(
        runtime: &JsRuntime,
        worker_count: usize,
    ) -> anyhow::Result<Self> {
        let (tx, rx) = channel();

        for i in 0..(worker_count * 2) {
            println!("[{}] Worker Started", i);

            let rx = rx.clone();
            let worker = runtime.spawn_worker()?;

            let _handle: JoinHandle<anyhow::Result<()>> = tokio::task::spawn(async move {
                let ctx = worker.create_context()?;

                while let Ok(callback) = rx.recv_async().await {
                    if ctx.exec(callback).is_err() {
                        eprintln!("Error communicating with JavaScript")
                    }
                }

                Ok(())
            });
        }

        Ok(Self { queue: tx })
    }

    pub fn exec(
        &self,
        callback: impl 'static + Send + FnOnce(&Env) -> ion::Result<()>,
    ) -> anyhow::Result<()> {
        self.queue.try_send(Box::new(callback)).unwrap();
        Ok(())
    }

    pub async fn exec_async<Return: 'static + Send + Sync>(
        &self,
        callback: impl 'static + Send + FnOnce(&Env) -> ion::Result<Return>,
    ) -> anyhow::Result<Return> {
        let (tx, rx) = oneshot();

        self.exec(move |env| Ok(tx.try_send(callback(env)?)?))?;

        let Ok(ret) = rx.recv_async().await else {
            return Err(anyhow::anyhow!("Failed to send"));
        };
        Ok(ret)
    }
}
