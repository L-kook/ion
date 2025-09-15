use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use crate::fs::FileSystem;

#[derive(Debug)]
pub struct ResolverContext {
    pub fs: FileSystem,
    pub specifier: String,
    pub from: PathBuf,
}

#[derive(Debug)]
pub struct ResolverResult {
    pub code: Vec<u8>,
    pub path: PathBuf,
    pub kind: String,
}

pub(crate) type DynResolver = Arc<dyn Send + Sync + Fn(ResolverContext) -> DynResolverFut>;
pub(crate) type DynResolverFut =
    Pin<Box<dyn Send + Sync + Future<Output = crate::Result<Option<ResolverResult>>>>>;
