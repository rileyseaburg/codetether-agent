//! Static Seatbelt (SBPL) rules shared by every macOS sandbox profile.

/// Deny-by-default rules that still permit process startup and reads.
///
/// Reads stay permitted because macOS toolchains resolve frameworks, dyld
/// caches, and trust stores far outside the workspace. Write access and
/// network access are the confined dimensions, and the read gap is reported
/// to callers through the sandbox result's isolation-gap list.
pub(super) const HEADER: &[&str] = &[
    "(version 1)",
    "(deny default)",
    "(allow process-exec)",
    "(allow process-fork)",
    "(allow signal (target self))",
    "(allow process-info* (target self))",
    "(allow sysctl-read)",
    "(allow mach-lookup)",
    "(allow file-read*)",
    "(allow file-ioctl)",
];

/// Character devices a confined process may still write to.
pub(super) const WRITABLE_DEVICES: &[&str] = &[
    "/dev/null",
    "/dev/zero",
    "/dev/stdout",
    "/dev/stderr",
    "/dev/tty",
];

/// Rule enabling outbound network access when the policy allows it.
pub(super) const ALLOW_NETWORK: &str = "(allow network*)";

/// Workspace-relative paths that stay read-only even inside allowed roots.
pub(super) const PROTECTED: &[&str] = &[".git", ".codetether", ".codex", ".agents"];
