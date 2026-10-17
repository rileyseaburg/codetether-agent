//! Crash reporting codetether data dir.
use super::*;
pub(super) fn codetether_data_dir() -> PathBuf {
    Config::data_dir().unwrap_or_else(|| PathBuf::from("/tmp/codetether-agent"))
}
