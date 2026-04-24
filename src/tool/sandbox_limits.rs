pub fn apply_memory_limit(cmd: &mut tokio::process::Command, max_bytes: u64) -> Vec<String> {
    if max_bytes == 0 {
        return Vec::new();
    }
    apply(cmd, max_bytes)
}

#[cfg(unix)]
fn apply(cmd: &mut tokio::process::Command, max_bytes: u64) -> Vec<String> {
    unsafe {
        cmd.pre_exec(move || {
            let limit = libc::rlimit {
                rlim_cur: max_bytes as libc::rlim_t,
                rlim_max: max_bytes as libc::rlim_t,
            };
            if libc::setrlimit(libc::RLIMIT_AS, &limit) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    Vec::new()
}

#[cfg(not(unix))]
fn apply(_cmd: &mut tokio::process::Command, _max_bytes: u64) -> Vec<String> {
    vec!["memory_limit_unenforced".to_string()]
}
