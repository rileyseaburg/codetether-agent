//! Install a seccomp cBPF program directly in a child before `exec`.

use std::sync::Arc;

pub(super) fn tokio(command: &mut tokio::process::Command, filters: Arc<Vec<u64>>) {
    unsafe {
        command.pre_exec(move || install(&filters));
    }
}

pub(super) fn blocking(command: &mut std::process::Command, filters: Arc<Vec<u64>>) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(move || install(&filters));
    }
}

fn install(filters: &[u64]) -> std::io::Result<()> {
    let length = u16::try_from(filters.len()).map_err(|_| std::io::Error::other("seccomp profile too large"))?;
    let mut profile = libc::sock_fprog {
        len: length,
        filter: filters.as_ptr().cast::<libc::sock_filter>().cast_mut(),
    };
    let no_new_privileges = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
    if no_new_privileges != 0 {
        return Err(std::io::Error::last_os_error());
    }
    let filtered = unsafe {
        libc::prctl(
            libc::PR_SET_SECCOMP,
            libc::SECCOMP_MODE_FILTER,
            &mut profile as *mut libc::sock_fprog,
        )
    };
    if filtered != 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}