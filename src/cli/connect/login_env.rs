//! Shell prefix that restores an interactive-login PATH for SSH commands.
//!
//! `codetether connect` runs commands over **non-interactive** SSH, which does
//! not source `~/.bashrc` or `~/.cargo/env`. A binary installed by
//! `cargo install --path .` lives in `~/.cargo/bin`, so without this prefix
//! `command -v codetether` resolves to a stale `/usr/local/bin/codetether`
//! left by an old `make install-*` deploy instead of the freshly built one.

/// Prefix that sources cargo's env and prepends `~/.cargo/bin` to `PATH`.
///
/// Prepend this (with a trailing `;`) to any remote command string so the
/// non-interactive shell finds the `cargo install`-produced binary first.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::connect::login_path_prefix;
///
/// let prefix = login_path_prefix();
/// assert!(prefix.contains(".cargo/bin"));
/// assert!(prefix.trim_end().ends_with(';'));
/// ```
pub fn login_path_prefix() -> &'static str {
    "[ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"; \
     export PATH=\"$HOME/.cargo/bin:$PATH\"; "
}
