pub fn apply_memory_limit(cmd: &mut tokio::process::Command, max_bytes: u64) -> Vec<String> {
    if max_bytes == 0 {
        return Vec::new();
    }
    apply(cmd, max_bytes)
}

#[cfg(unix)]
fn apply(cmd: &mut tokio::process::Command, max_bytes: u64) -> Vec<String> {
    let max_rlim = libc::rlim_t::MAX;
    let mut fallbacks = Vec::new();
    let limit_bytes = if u128::from(max_bytes) > max_rlim as u128 {
        fallbacks.push("memory_limit_clamped".to_string());
        max_rlim
    } else {
        max_bytes as libc::rlim_t
    };
    unsafe {
        cmd.pre_exec(move || {
            let limit = libc::rlimit {
                rlim_cur: limit_bytes,
                rlim_max: limit_bytes,
            };
            if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    fallbacks
}

#[cfg(not(unix))]
fn apply(_cmd: &mut tokio::process::Command, max_bytes: u64) -> Vec<String> {
    tracing::warn!(
        max_bytes,
        "Sandbox memory limit cannot be enforced on this platform; \
         caller will see 'memory_limit_unenforced' in unsafe_fallbacks"
    );
    vec!["memory_limit_unenforced".to_string()]
}
