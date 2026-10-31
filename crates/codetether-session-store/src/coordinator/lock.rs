//! Advisory exclusion between processes using the same session database.
use super::CoordinationError;
use anyhow::Result;
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::Path,
    time::{Duration, Instant},
};

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
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => return Ok(Guard(file)),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
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
