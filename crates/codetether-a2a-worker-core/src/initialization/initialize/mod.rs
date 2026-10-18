//! Shared worker initialization and native service bindings.
//!
//! [init_worker] constructs the identity, shared states and task runtime, while
//! [Host] binds application services. [Options] borrows native CLI settings.
mod host;
mod options;
mod run;
pub use host::Host;
pub use options::Options;
pub use run::init_worker;
