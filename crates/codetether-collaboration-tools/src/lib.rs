//! First-class collaboration lifecycle tools and trusted context forwarding.
//!
//! [RuntimeContext] preserves injected session, workspace and model values.
//! Implement [Host] with an existing agent runtime, then construct the spawn,
//! interrupt, close or resume tool. [execute] forwards legacy payloads after
//! injecting the same trusted context.
mod close;
mod context;
mod host;
mod interrupt;
mod legacy;
mod resume;
mod spawn;
pub use close::CloseAgentTool;
pub use context::RuntimeContext;
pub use host::Host;
pub use interrupt::InterruptAgentTool;
pub use legacy::execute;
pub use resume::ResumeAgentTool;
pub use spawn::SpawnAgentTool;
