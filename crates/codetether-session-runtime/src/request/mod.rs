//! Owned prompt requests and runtime command messages.
//! [PromptRequest] moves session state into execution. [SessionCommand] carries
//! a boxed request or lightweight control signal. [SessionIdentity] supplies the
//! identifier used by the request's compact debug representation.
mod command;
mod debug;
mod identity;
mod new;
mod state;
pub use command::SessionCommand;
pub use identity::SessionIdentity;
pub use state::PromptRequest;
