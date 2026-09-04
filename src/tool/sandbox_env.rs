use std::collections::HashMap;
use std::path::PathBuf;

pub(super) fn restricted() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("PATH".to_string(), search_path());
    env.insert("HOME".to_string(), "/tmp".to_string());
    env.insert("LANG".to_string(), "C.UTF-8".to_string());
    inject_runtime_env(&mut env);
    env
}

/// Host `PATH` entries under system or exposed toolchain roots, in host
/// order, followed by the baseline system directories.
fn search_path() -> String {
    let roots = super::sandbox_toolchain::roots();
    let mut entries = super::sandbox_toolchain::path_entries(&roots);
    for baseline in ["/usr/bin", "/bin"] {
        let path = PathBuf::from(baseline);
        if !entries.contains(&path) {
            entries.push(path);
        }
    }
    std::env::join_paths(entries)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string())
}

fn inject_runtime_env(env: &mut HashMap<String, String>) {
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
