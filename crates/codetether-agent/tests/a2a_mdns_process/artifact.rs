use std::path::{Path, PathBuf};

pub enum ProofDirectory {
    Temporary(tempfile::TempDir),
    Persistent(PathBuf),
}

impl ProofDirectory {
    pub fn new() -> Self {
        match std::env::var_os("CODETETHER_MDNS_PROOF_DIR") {
            Some(path) => {
                let path = PathBuf::from(path);
                std::fs::create_dir_all(&path).expect("create persistent proof directory");
                Self::Persistent(path)
            }
            None => Self::Temporary(tempfile::tempdir().expect("create proof directory")),
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::Temporary(directory) => directory.path(),
            Self::Persistent(directory) => directory,
        }
    }
}
