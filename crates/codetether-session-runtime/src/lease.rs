//! Run identity shared by frontend reservations and executors.

/// Ownership of one generation of a session run.
///
/// Obtain leases through [`crate::ControlPlane::reserve`] or
/// [`crate::ControlPlane::begin`]; callers cannot construct arbitrary owners.
///
/// # Examples
/// ```
/// use codetether_session_runtime::ControlPlane;
/// let runtime = ControlPlane::default();
/// let lease = runtime.reserve("session").unwrap();
/// assert_eq!(lease.session_id(), "session");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lease {
    pub(crate) session_id: String,
    pub(crate) generation: u64,
}

impl Lease {
    /// Session targeted by this particular run reservation.
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}
