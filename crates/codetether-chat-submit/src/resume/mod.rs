//! Manual continuation of stalled or idle chat turns.
//!
//! [run] recognizes the command, resolves its prompt through [Source] and
//! chooses idle dispatch or stalled cancellation/retry preparation. [Ui] keeps
//! concrete interface state in the caller; [Runtime] supplies execution effects.
mod idle;
mod prompt;
mod run;
mod source;
mod stalled;
mod ui;
pub use prompt::resolve;
pub use run::run;
pub use source::Source;
use std::future::Future;
pub use ui::Ui;
/// Runtime effects used after continuation state has been prepared.
/// # Implementors
/// Native adapters submit prompts and cancel the current runtime request.
pub trait Runtime {
    /// Dispatch an idle continuation without new image attachments.
    fn dispatch(&mut self, prompt: &str) -> impl Future<Output = ()>;
    /// Cancel the active request after arming retry state.
    fn cancel(&self) -> impl Future<Output = ()>;
}
/// Complete native continuation interface.
/// # Implementors
/// Types providing prompt lookup, UI effects and runtime effects are hosts.
pub trait Host: Source + Ui + Runtime {}
impl<T: Source + Ui + Runtime> Host for T {}
