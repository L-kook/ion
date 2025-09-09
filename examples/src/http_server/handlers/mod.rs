use std::path::PathBuf;

use tokio::fs;

static CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub async fn get_handler(path: &str) -> anyhow::Result<String> {
    let handler_root = PathBuf::from(CARGO_MANIFEST_DIR)
        .join("src")
        .join("http_server")
        .join("handlers");

    let mut path = path;
    if path == "/" {
        path = "/root"
    }

    let mut target = handler_root;
    for segment in path.split("/") {
        target = target.join(segment)
    }
    target.set_extension("js");

    Ok(fs::read_to_string(target).await?)
}
