//! Optional process-wide audit log initialization and access.

use crate::AuditLog;

static AUDIT_LOG: tokio::sync::OnceCell<AuditLog> = tokio::sync::OnceCell::const_new();

/// Initialize the global audit log once.
///
/// # Arguments
/// * `log` — The log to share for the remainder of the process.
///
/// # Returns
/// `Ok(())` on first initialization.
///
/// # Errors
/// Returns a fresh default log if initialization has already occurred (not the
/// supplied log); the existing global log is unchanged.
///
/// # Examples
/// ```
/// use codetether_audit::{AuditLog, init_audit_log, try_audit_log};
/// assert!(init_audit_log(AuditLog::default()).is_ok());
/// assert!(try_audit_log().is_some());
/// ```
pub fn init_audit_log(log: AuditLog) -> Result<(), AuditLog> {
    AUDIT_LOG.set(log).map_err(|_| AuditLog::default())
}

/// Get the global audit log.
///
/// # Arguments
/// None.
/// # Returns
/// The process-wide log initialized by [`init_audit_log`].
/// # Panics
/// Panics if [`init_audit_log`] has not been called successfully.
/// # Examples
/// ```
/// use codetether_audit::{AuditLog, audit_log, init_audit_log};
/// init_audit_log(AuditLog::default()).unwrap();
/// assert!(std::ptr::eq(audit_log(), audit_log()));
/// ```
pub fn audit_log() -> &'static AuditLog {
    AUDIT_LOG
        .get()
        .expect("Audit log not initialized — call init_audit_log() at startup")
}

/// Get the global audit log without panicking before initialization.
///
/// # Arguments
/// None.
/// # Returns
/// `Some(log)` after initialization, otherwise `None`.
/// # Examples
/// ```
/// assert!(codetether_audit::try_audit_log().is_none());
/// ```
pub fn try_audit_log() -> Option<&'static AuditLog> {
    AUDIT_LOG.get()
}
