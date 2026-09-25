//! Minimal bubblewrap probe filesystem, including the dynamic ELF loader.
//!
//! Used by the sandbox availability probe to execute the host's `true` binary.

/// Probe arguments mirror the runtime's read-only library roots.
pub(in crate::tool) const SMOKE_ARGS: &[&str] = &[
    "--die-with-parent",
    "--new-session",
    "--unshare-user-try",
    "--unshare-ipc",
    "--unshare-pid",
    "--ro-bind",
    "/usr",
    "/usr",
    "--ro-bind-try",
    "/bin",
    "/bin",
    "--ro-bind-try",
    "/lib",
    "/lib",
    "--ro-bind-try",
    "/lib64",
    "/lib64",
    "--proc",
    "/proc",
    "--dev",
    "/dev",
    "--tmpfs",
    "/tmp",
    "--",
    "/usr/bin/true",
];