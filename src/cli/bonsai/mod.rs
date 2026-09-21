//! Direct native Bonsai command: local checkpoint -> CUDA decoder -> streamed text.
//! [`BonsaiArgs`] configure inference; [`run`] executes it without agent/session startup.
mod args;
mod request;
mod run;
pub use args::BonsaiArgs;
pub use run::execute as run;
