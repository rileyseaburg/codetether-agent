//! Deterministic page download records.
//!
//! The module stores downloads on [`BrowserPage`](crate::browser_agent::BrowserPage)
//! without touching the host filesystem.

mod anchor;
mod filename;
mod page;
mod record;

#[cfg(test)]
mod tests;

pub(crate) use anchor::{click_script, is_anchor_download, record_anchor_download};
pub use record::{DownloadRecord, DownloadStatus};
