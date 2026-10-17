//! Expose the active CodeTether executable to sandboxed child commands.

use std::collections::HashMap;

pub(super) fn extend(env: &mut HashMap<String, String>) {
    let Ok(current_exe) = std::env::current_exe() else {
        return;
    };
    env.insert(
        "CODETETHER_BIN".to_string(),
        current_exe.to_string_lossy().into_owned(),
    );
    let mut entries = current_exe
        .parent()
        .map(|parent| vec![parent.to_path_buf()])
        .unwrap_or_default();
    if let Some(existing) = env.get("PATH").map(std::ffi::OsString::from) {
        entries.extend(std::env::split_paths(&existing));
    }
    if let Ok(path) = std::env::join_paths(entries) {
        env.insert("PATH".to_string(), path.to_string_lossy().into_owned());
    }
}
