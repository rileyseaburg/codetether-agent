//! Existing bounded executor regression.
use super::*;

#[cfg(target_os = "linux")]
pub(super) fn thread_names() -> Vec<String> {
    std::fs::read_dir("/proc/self/task")
        .expect("read process tasks")
        .filter_map(Result::ok)
        .filter_map(|entry| std::fs::read_to_string(entry.path().join("comm")).ok())
        .map(|name| name.trim().to_string())
        .collect()
}
