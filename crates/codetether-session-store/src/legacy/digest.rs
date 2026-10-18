//! Bounded digest of legacy input and its preserved backup.
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{io::Read, path::Path};

pub(super) fn digest(path: &Path) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    anyhow::ensure!(
        file.metadata()?.len() <= 512 * 1024 * 1024,
        "legacy session exceeds 512 MiB import cap"
    );
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().to_vec())
}
