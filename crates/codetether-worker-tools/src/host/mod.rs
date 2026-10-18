//! Application bindings needed by worker tool assembly.
mod registry;
mod workflow;
mod workspace;
pub use registry::Registry;
pub use workflow::Workflow;
pub use workspace::Workspace;
/// Complete application binding for worker tool assembly.
/// # Implementors
/// Any adapter implementing [Registry], [Workflow] and [Workspace].
pub trait Host: Registry + Workflow + Workspace {}
impl<T: Registry + Workflow + Workspace> Host for T {}
