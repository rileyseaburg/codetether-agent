//! Runtime exception taxonomy for production diagnostics.

/// High-level category for a runtime failure.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RuntimeExceptionKind;
///
/// let kind = RuntimeExceptionKind::Reference;
/// assert!(matches!(kind, RuntimeExceptionKind::Reference));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeExceptionKind {
    /// Missing binding or unresolved identifier.
    Reference,
    /// Invalid operation for the runtime value type.
    Type,
    /// Parse or syntax failure before execution.
    Syntax,
    /// Failed request, blocked request, or network abort.
    Network,
    /// Browser permission denial.
    Permission,
    /// Failure that does not fit a known category yet.
    Other,
}
