//! Last-loaded trust-policy snapshot and access-mode notification.
use codetether_config::{Config, TrustPolicyStatus};
use std::sync::{LazyLock, Mutex};
static STATUS: LazyLock<Mutex<Option<TrustPolicyStatus>>> = LazyLock::new(|| Mutex::new(None));

/// Store the loaded config's effective trust policy status for TUI rendering.
///
/// # Arguments
/// * `config` - Loaded effective configuration.
///
/// # Returns
/// Updates the stored snapshot and shared access-mode observation.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// codetether_trust_status::set_from_config(&codetether_config::Config::default());
/// ```
pub fn set_from_config(config: &Config) {
    set_status(TrustPolicyStatus::from_config(config));
}

/// Store an explicit trust policy status.
///
/// # Arguments
/// * `status` - Effective trust-policy snapshot.
///
/// # Returns
/// Updates the stored snapshot and shared access-mode observation.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// let config = codetether_config::Config::default();
/// codetether_trust_status::set_status(codetether_config::TrustPolicyStatus::from_config(&config));
/// ```
pub fn set_status(status: TrustPolicyStatus) {
    codetether_runtime_policy::justification_schema::note_access_mode(status.access_mode);
    *STATUS.lock().expect("trust status mutex poisoned") = Some(status);
}

/// Return the last loaded trust policy status.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// The last stored snapshot, or none before initialization.
///
/// # Panics
/// Panics if the shared status mutex is poisoned.
///
/// # Examples
/// ```
/// let _snapshot = codetether_trust_status::current_status();
/// ```
pub fn current_status() -> Option<TrustPolicyStatus> {
    *STATUS.lock().expect("trust status mutex poisoned")
}
