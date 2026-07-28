//! Classified runtime exception records.

use super::exception_kind::RuntimeExceptionKind;

/// Runtime failure classified for agent triage.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RuntimeException, RuntimeExceptionKind};
///
/// let exception = RuntimeException {
///     action: "page.eval_js".into(),
///     message: "ReferenceError: missing is not defined".into(),
///     kind: RuntimeExceptionKind::Reference,
/// };
/// assert_eq!(exception.kind, RuntimeExceptionKind::Reference);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeException {
    /// Action or subsystem that produced the failure.
    pub action: String,
    /// Original diagnostic message.
    pub message: String,
    /// Classified failure category.
    pub kind: RuntimeExceptionKind,
}
