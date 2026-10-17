use super::*;

#[async_trait]
impl DurableLog for FileDurableLog {
    async fn append(&self, env: &BusEnvelope) -> Result<u64> {
        let path = file_path::partition_path(&self.dir, partition_of(env));
        let mut line = serde_json::to_string(env).context("serialize envelope")?;
        line.push('\n');
        let _guard = self.write_lock.lock().await;
        fs::create_dir_all(&self.dir).await.ok();
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await
            .with_context(|| format!("open {}", path.display()))?;
        f.write_all(line.as_bytes()).await.context("append line")?;
        // Tokio may return from write_all before its blocking write has finished.
        // Complete it before a separate reader computes the committed offset.
        f.flush().await.context("flush appended line")?;
        file_read::count_lines(&path).await
    }

    async fn tail(&self, partition: &str, after: u64) -> Result<Vec<BusEnvelope>> {
        let path = file_path::partition_path(&self.dir, partition);
        file_read::read_after(&path, after).await
    }
}
