//! File identity and folded state for incremental journal replay.
use codetether_goal_core::TaskState;
use std::{
    fs::{File, Metadata},
    sync::Arc,
    time::SystemTime,
};

#[derive(Clone)]
pub(super) struct Entry {
    // Retain the file object so an unlinked inode cannot be recycled while its
    // identity is cached. The enclosing LRU bounds the number of open handles.
    pub(super) _source: Arc<File>,
    pub(super) identity: Identity,
    pub(super) length: u64,
    pub(super) modified: Option<SystemTime>,
    pub(super) state: TaskState,
    #[cfg(test)]
    pub(super) replayed_bytes: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub(super) struct Identity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(not(unix))]
    created: Option<SystemTime>,
}

impl Identity {
    pub(super) fn reliable(&self) -> bool {
        #[cfg(unix)]
        {
            true
        }
        #[cfg(not(unix))]
        {
            self.created.is_some()
        }
    }

    pub(super) fn from_metadata(metadata: &Metadata) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            Self {
                device: metadata.dev(),
                inode: metadata.ino(),
            }
        }
        #[cfg(not(unix))]
        {
            Self {
                created: metadata.created().ok(),
            }
        }
    }
}
