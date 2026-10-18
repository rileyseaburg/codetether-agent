//! Best-effort JSONL append without blocking the prompt loop.

pub(super) async fn append(path: std::path::PathBuf, json: String) {
    use tokio::io::AsyncWriteExt;
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await;
    if let Ok(mut file) = file {
        let _ = file.write_all(json.as_bytes()).await;
        let _ = file.write_all(b"\n").await;
    }
}
