//! Process-wide non-interactive hardening for spawned subprocesses.
//!
//! Every `git`/shell child inherits this environment, so no subprocess
//! can block the TUI (or a worker) with an interactive credential prompt
//! such as `Username for 'https://...':`. Values already present in the
//! environment are respected and never overwritten.

/// Environment defaults that force child processes to fail fast instead
/// of prompting on a terminal.
const DEFAULTS: &[(&str, &str)] = &[
    // Git: never prompt on the controlling terminal; fail instead.
    ("GIT_TERMINAL_PROMPT", "0"),
    // Git: if a prompt would still be attempted, route it to a failing
    // askpass instead of the TTY (covers PTY-inheriting children).
    ("GIT_ASKPASS", "/bin/false"),
    // Git Credential Manager: never pop interactive UI.
    ("GCM_INTERACTIVE", "never"),
    // SSH: refuse askpass UI prompts.
    ("SSH_ASKPASS", "/bin/false"),
    ("SSH_ASKPASS_REQUIRE", "never"),
];

/// Apply non-interactive defaults to the current process environment.
///
/// Call this once, as early as possible in `main()` (before spawning
/// threads or subprocesses). Existing variables are left untouched so
/// operators can still opt into custom askpass/credential flows.
pub fn harden() {
    for (key, value) in DEFAULTS {
        if std::env::var_os(key).is_none() {
            // SAFETY: called once during single-threaded startup, before
            // the async runtime or any subprocess is spawned.
            unsafe { std::env::set_var(key, value) };
        }
    }
}

/// Load local defaults, harden child-process prompts and initialize cryptography.
/// # Arguments
/// Reads the process environment and optional local dotenv file.
/// # Returns
/// Completes after TLS and JWT provider installation.
/// # Errors
/// Returns an error when required FIPS mode is not active.
/// # Examples
/// ```no_run
/// codetether_cli_startup::environment::initialize().unwrap();
/// ```
pub fn initialize() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    harden();
    codetether_tls::ensure_rustls_crypto_provider();
    let _ = jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default();
    codetether_tls::require_fips()?;
    Ok(())
}
