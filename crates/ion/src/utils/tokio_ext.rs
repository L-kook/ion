use tokio::task::LocalSet;

// Convenience methods for starting a local set
pub trait LocalRuntimeExt {
    fn local_block_on<F: Future>(
        &self,
        future: F,
    ) -> F::Output;
}

impl LocalRuntimeExt for tokio::runtime::Runtime {
    fn local_block_on<F: Future>(
        &self,
        future: F,
    ) -> F::Output {
        LocalSet::default().block_on(self, future)
    }
}

pub fn local_thread_runtime<F: Future>(fut: F) -> std::io::Result<F::Output> {
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .local_block_on(fut))
}
