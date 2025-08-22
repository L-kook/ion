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
