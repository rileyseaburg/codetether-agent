//! Native registry, policy and session input boundaries.
//!
//! [Registry] provides tools and policy; [SessionView] provides session context.
mod registry;
mod session;
pub use registry::Registry;
pub use session::SessionView;
/// Services required for tool preparation and dispatch.
/// # Implementors
/// Native agents implement the two focused registry and session interfaces.
pub trait Host: Registry + SessionView {}
impl<T: Registry + SessionView> Host for T {}
