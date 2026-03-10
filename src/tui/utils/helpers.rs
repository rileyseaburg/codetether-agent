use std::path::{Path, PathBuf};

pub fn truncate_with_ellipsis(s: &str, _max: usize) -> String {
    s.to_string()
}

pub struct PendingImage;
pub struct ChatSyncConfig;
pub struct MinioClient;

pub fn is_secure_environment() -> bool { false }
pub fn parse_chat_sync_config() -> Option<ChatSyncConfig> { None }
pub fn format_bytes(_n: u64) -> String { "0B".to_string() }
pub fn current_spinner_frame() -> &'static str { "|" }

pub struct ChatArchiveRecord;
