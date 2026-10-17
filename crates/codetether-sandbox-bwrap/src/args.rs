//! args for bubblewrap argument and mount planning.
use super::{SandboxPolicy, paths, push::*};
use std::path::Path;

/// Build ordered bubblewrap arguments for a command and filesystem policy.
///
/// # Arguments
/// * `command` - Executable passed after bubblewrap options.
/// * `args` - Arguments for that executable.
/// * `policy` - Writable roots and network permission.
/// * `work_dir` - Directory used inside the sandbox.
/// * `seccomp_fd` - Optional inherited filter descriptor.
/// # Returns
/// Bubblewrap arguments with runtime mounts, protected metadata and the command.
/// # Examples
/// ```rust,no_run
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// let args = codetether_sandbox_bwrap::build(
///     "example", &[], &policy, std::path::Path::new("/workspace"), None,
/// );
/// assert!(args.contains(&"--die-with-parent".to_string()));
/// ```
pub fn build(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &Path,
    seccomp_fd: Option<i32>,
) -> Vec<String> {
    let mut out = base_args(policy.allow_network);
    if let Some(fd) = seccomp_fd {
        push_pair(&mut out, "--seccomp", &fd.to_string());
    }
    mount_runtime(&mut out);
    paths::mounts(&mut out, policy, work_dir);
    push_pair(&mut out, "--chdir", &work_dir.display().to_string());
    out.push("--".to_string());
    out.push(command.to_string());
    out.extend(args.iter().cloned());
    out
}

fn base_args(allow_network: bool) -> Vec<String> {
    let mut out = [
        "--die-with-parent",
        "--new-session",
        "--unshare-user-try",
        "--unshare-ipc",
        "--unshare-pid",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    if !allow_network {
        out.push("--unshare-net".to_string());
    }
    out
}

fn mount_runtime(out: &mut Vec<String>) {
    push_pair(out, "--proc", "/proc");
    push_pair(out, "--dev", "/dev");
    push_pair(out, "--tmpfs", "/tmp");
    push_triple(out, "--ro-bind", "/usr", "/usr");
    for path in ["/bin", "/sbin", "/lib", "/lib64", "/etc"] {
        push_triple(out, "--ro-bind-try", path, path);
    }
}
