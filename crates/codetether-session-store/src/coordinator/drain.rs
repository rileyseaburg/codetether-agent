//! Open the current database for each queued operation under the process lock.
//! A cached connection can outlive a replaced data directory or schema downgrade.
use super::{job::Job, lock};
use std::{path::PathBuf, sync::mpsc::Receiver};

pub(super) fn run(path: PathBuf, receiver: Receiver<Job>) {
    while let Ok(job) = receiver.recv() {
        let _guard = match lock::acquire(&path) {
            Ok(guard) => guard,
            Err(error) => {
                job(Err(error));
                continue;
            }
        };
        match crate::connection::writer(&path) {
            Ok(mut db) => job(Ok(&mut db)),
            Err(error) => job(Err(error)),
        }
    }
}
