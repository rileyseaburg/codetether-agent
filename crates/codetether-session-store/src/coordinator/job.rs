//! Owned operations, panic isolation, rollback, and post-commit replies.
use super::CoordinationError;
use anyhow::Result;
use rusqlite::Connection;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub(super) type Job = Box<dyn FnOnce(Result<&mut Connection>) + Send>;

pub(super) fn new<T: Send + 'static>(
    operation: impl FnOnce(&mut Connection) -> Result<T> + Send + 'static,
    reply: impl FnOnce(Result<T>) + Send + 'static,
) -> Job {
    Box::new(move |connection| {
        reply(connection.and_then(|db| apply(db, operation)));
    })
}

fn apply<T>(
    db: &mut Connection,
    operation: impl FnOnce(&mut Connection) -> Result<T>,
) -> Result<T> {
    let result = catch_unwind(AssertUnwindSafe(|| operation(db)))
        .unwrap_or_else(|_| Err(CoordinationError::Panicked.into()));
    if !db.is_autocommit() {
        db.execute_batch("ROLLBACK")?;
        return match result {
            Err(error) => Err(error),
            Ok(_) => Err(CoordinationError::Uncommitted.into()),
        };
    }
    result
}
