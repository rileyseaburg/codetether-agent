//! Compatibility constants historically exposed by the native TUI.
//!
//! Worker configuration uses [`crate::config_types`]. These frontend values
//! remain separate because their interval and prefix defaults differ.
/// Historical frontend default interval between synchronization attempts.
pub const CHAT_SYNC_DEFAULT_INTERVAL_SECS: u64 = 15;
/// Historical frontend maximum synchronization interval.
pub const CHAT_SYNC_MAX_INTERVAL_SECS: u64 = 300;
/// Historical frontend upper bound for a synchronization batch.
pub const CHAT_SYNC_MAX_BATCH_BYTES: usize = 512 * 1024;
/// Historical frontend archive bucket.
pub const CHAT_SYNC_DEFAULT_BUCKET: &str = "codetether-chat-archive";
/// Historical frontend archive object prefix.
pub const CHAT_SYNC_DEFAULT_PREFIX: &str = "sessions";
