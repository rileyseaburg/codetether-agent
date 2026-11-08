//! Host-local refresh coordination; lock files contain no credentials.
use super::Store;
use anyhow::{Result, anyhow};
use fs2::FileExt;
use std::{
    fs::{self, File, OpenOptions},
    time::Duration,
};

impl Store {
    pub(crate) async fn lock(&self) -> Result<File> {
        let directories = directories::ProjectDirs::from("ai", "codetether", "codetether-agent")
            .ok_or_else(|| anyhow!("CodeTether local-data directory is unavailable"))?;
        let directory = directories.data_local_dir().join("locks");
        fs::create_dir_all(&directory)
            .map_err(|_| anyhow!("Could not prepare Monarch refresh lock"))?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(directory.join("monarch-oauth-refresh.lock"))
            .map_err(|_| anyhow!("Could not open Monarch refresh lock"))?;
        tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                match file.try_lock_exclusive() {
                    Ok(()) => return Ok(()),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    Err(_) => return Err(anyhow!("Could not acquire Monarch refresh lock")),
                }
            }
        })
        .await
        .map_err(|_| {
            anyhow!("Another Monarch authentication operation is active; retry shortly")
        })??;
        Ok(file)
    }
}
