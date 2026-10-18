//! Process-wide owner for frontend reservations and session executors.
/// Single control plane shared by frontend reservations and native executors.
/// # Examples
/// ```rust
/// let cancelled = codetether_session_control::CONTROL.is_cancelled("no-such-session");
/// assert!(!cancelled);
/// ```
pub static CONTROL: std::sync::LazyLock<codetether_session_runtime::ControlPlane> =
    std::sync::LazyLock::new(codetether_session_runtime::ControlPlane::default);
