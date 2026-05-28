//! Clone lock release helpers.

use super::git_clone_base_dir;

pub(super) async fn release_workspace_clone_lock(lock: tokio::fs::File) {
    if let Ok(metadata) = lock.metadata().await {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(mut entries) = tokio::fs::read_dir(git_clone_base_dir()).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(entry_metadata) = entry.metadata().await
                        && entry_metadata.ino() == metadata.ino()
                        && entry_metadata.dev() == metadata.dev()
                    {
                        drop(lock);
                        let _ = tokio::fs::remove_file(entry.path()).await;
                        return;
                    }
                }
            }
        }
    }
    drop(lock);
}
