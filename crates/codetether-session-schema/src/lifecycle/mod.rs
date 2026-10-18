//! Application-local lifecycle declarations grouped by responsibility.
//! [crate::impl_session_lifecycle!] adds construction and shared state operations.
mod bus;
mod construct;
mod history;
mod provenance;

/// Add shared lifecycle methods to a type declared with [crate::declare_session!].
///
/// The application retains its own provider and policy methods on this local type.
/// # Examples
/// ```no_run
/// codetether_session_schema::declare_session!(pub struct Session;);
/// codetether_session_schema::impl_session_lifecycle!(Session);
/// let mut session = Session::default();
/// session.set_agent_name("build");
/// assert_eq!(session.agent, "build");
/// ```
#[macro_export]
macro_rules! impl_session_lifecycle {
    ($name:ident) => {
        $crate::__session_lifecycle_construct!($name);
        $crate::__session_lifecycle_bus!($name);
        $crate::__session_lifecycle_provenance!($name);
        $crate::__session_lifecycle_history!($name);
    };
}
