use std::path::PathBuf;

pub struct ResolverContext {
    pub specifier: String,
    pub from: PathBuf,
}

pub struct ResolverResult {
    pub code: Vec<u8>,
    pub kind: String,
}
