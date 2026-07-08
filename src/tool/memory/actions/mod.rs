//! Action handlers for the [`MemoryTool`](super::MemoryTool) `execute`
//! dispatch, split per SRP: one file per action group.
//!
//! * [`save_delete`] — mutations (save, delete)
//! * [`retrieve`] — single-entry retrieval (get)
//! * [`find`] — search
//! * [`list`] — recent-entry listing
//! * [`report`] — tags and stats

mod find;
mod list;
mod report;
mod retrieve;
mod save_delete;
