//! Ordered process startup with native parser and command-dispatch bindings.
//!
//! Implement [Host] and call [run] inside the application's runtime. Native
//! early commands retain their bypass before environment and service setup.
mod early;
mod host;
mod run;
pub use host::Host;
pub use run::run;
