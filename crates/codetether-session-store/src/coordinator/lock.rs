//! Advisory exclusion between processes using the same session database.
use super::CoordinationError;
use anyhow::Result;
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::Path,
    time::{Duration, Instant},
};

/// Longest wait for other processes. Durable commits on a slow disk hold the
/// lock for seconds each and run back to back, so a short wait fails spuriously.
const PATIENCE: Duration = Duration::from_secs(30);
/// Waits beyond this are logged so contention stays observable.
const SLOW: Duration = Duration::from_secs(5);

pub(super) struct Guard(File);

pub(super) fn acquire(database: &Path) -> Result<Guard> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(database.with_extension("sqlite3.writer.lock"))?;
    let started = Instant::now();
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => {
                let waited = started.elapsed();
                if waited >= SLOW {
                    let waited_ms = waited.as_millis() as u64;
                    tracing::warn!(waited_ms, "Session writer lock was contended");
                }
                return Ok(Guard(file));
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if started.elapsed() >= PATIENCE {
                    return Err(CoordinationError::Busy.into());
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(error) => return Err(error.into()),
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if let Err(error) = FileExt::unlock(&self.0) {
            tracing::warn!(%error, "Session writer lock release failed");
        }
    }
}
