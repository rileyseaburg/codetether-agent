//! Claimed-task execution, event sinks, checkpoints and result finalization.
//! [execute] sequences special tasks and prepared sessions through native
//! [Sessions], [Events] and [Special] hooks without changing task policy.
mod events;
mod run;
mod sessions;
mod special;
pub use events::{Events, Output};
pub use run::execute;
pub use sessions::Sessions;
pub use special::Special;
/// Combined native services for a claimed task.
/// # Implementors
/// Worker runtime adapters supply session, event and special-task operations.
pub trait Host: Sessions + Events + Special {}
impl<T: Sessions + Events + Special> Host for T {}

mod finish;
mod prepare;
