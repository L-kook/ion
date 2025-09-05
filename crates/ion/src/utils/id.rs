use std::sync::LazyLock;
use std::sync::atomic::AtomicU64;

static ID: LazyLock<AtomicU64> = LazyLock::new(|| Default::default()); //Default::default();

pub fn new_id() -> u64 {
    ID.fetch_add(1, std::sync::atomic::Ordering::AcqRel)
}
