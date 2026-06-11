use std::collections::HashMap;

pub(super) fn restricted() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("PATH".to_string(), "/usr/bin:/bin".to_string());
    env.insert("HOME".to_string(), "/tmp".to_string());
    env.insert("LANG".to_string(), "C.UTF-8".to_string());
    inject_runtime_env(&mut env);
    env
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
    if let Some(existing) = env
        .get("PATH")
        .map(std::ffi::OsString::from)
        .or_else(|| std::env::var_os("PATH"))
    {
        entries.extend(std::env::split_paths(&existing));
    }
    if let Ok(path) = std::env::join_paths(entries) {
        env.insert("PATH".to_string(), path.to_string_lossy().into_owned());
    }
}
