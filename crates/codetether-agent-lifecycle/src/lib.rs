//! Durable child close/resume and resident subtree shutdown.
//!
//! Implement [Host] for application persistence, live sessions, and activation.
//! [close] closes a durable root and shuts down its resident subtree while
//! retaining descendant durable edges. [resume] activates only the selected
//! root with [ResumeConfig]. Transition locks and execution notifications use
//! the existing shared collaboration-state registries.
mod close;
mod host;
mod result;
mod resume;
mod resume_config;
mod shutdown;
mod wait;
pub use close::run as close;
pub use host::Host;
pub use resume::run as resume;
pub use resume_config::ResumeConfig;

mod control_host;
mod interrupt;
mod kill;
mod marker;
mod marker_host;
pub use control_host::ControlHost;
pub use interrupt::interrupt;
pub use kill::kill;
pub use marker::INTERRUPTION_MARKER;
pub use marker_host::MarkerHost;
