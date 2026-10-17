//! Well-known per-user toolchain roots, relative to `$HOME`.
//!
//! Each entry is a directory that installers create for language runtimes and
//! package managers. Only directories that exist on the host are exposed, and
//! always read-only, so listing an absent tool here is harmless.

/// Toolchain roots relative to the user's home directory.
pub(super) const RELATIVE: &[&str] = &[
    ".nvm/versions/node",
    ".local/share/pnpm",
    ".local/share/fnm",
    ".volta",
    ".bun",
    ".deno",
    ".cargo/bin",
    ".rustup/toolchains",
    ".local/bin",
    ".pyenv",
    ".rbenv",
    ".asdf",
    ".mise",
    "go/bin",
];
