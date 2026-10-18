//! Command consumption and ordered prompt submission gates.
//! [run] consumes commands until shutdown. [handle] dispatches one command;
//! [submit] checks review holds, attaches cancellation, announces and spawns.
//! [Host] retains native policy checks, notices and execution.
mod host;
mod run;
mod step;
mod submit;
pub use host::Host;
pub use run::run;
pub use step::handle;
pub use submit::submit;
