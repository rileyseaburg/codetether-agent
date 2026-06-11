use super::{SandboxPolicy, sandbox_bwrap_paths, sandbox_bwrap_push::*};
use std::path::Path;

pub(super) fn build(
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
    sandbox_bwrap_paths::mounts(&mut out, policy, work_dir);
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
