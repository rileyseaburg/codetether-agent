//! Process queued operations using a persistent connection under the process lock.
use super::{job::Job, lock};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
    },
};

pub(super) fn run(path: PathBuf, ready: Arc<AtomicBool>, receiver: Receiver<Job>) {
    let mut connection = None;
    while let Ok(job) = receiver.recv() {
        let _guard = match lock::acquire(&path) {
            Ok(guard) => guard,
            Err(error) => {
                job(Err(error));
                continue;
            }
        };
        if connection.is_none() {
            match crate::connection::writer(&path) {
                Ok(db) => connection = Some(db),
                Err(error) => {
                    job(Err(error));
                    continue;
                }
            }
        }
        ready.store(true, Ordering::Release);
        job(Ok(connection.as_mut().expect("writer initialized above")));
    }
}
