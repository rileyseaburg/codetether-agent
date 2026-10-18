//! Worker completion with goal prompt composition and durable usage accounting.
//! [complete] chooses the established sampling temperature, calls the native
//! completion path and records usage before returning the response. [Host]
//! supplies native goal and completion services.
mod host;
mod run;
pub use host::Host;
pub use run::complete;
