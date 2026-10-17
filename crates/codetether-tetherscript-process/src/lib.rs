//! TetherScript subprocess authority with bounded output and progress delivery.
//!
//! [ProcessAuthority] supplies the native execution capability, while [inject]
//! installs the language-level wrapper. Argument parsing, child I/O, timeout
//! handling, result arity and progress delivery live in separate modules.
mod process_args;
mod process_authority;
mod process_io;
mod process_output;
mod process_prelude;
mod process_result;
mod process_run;
mod process_spawn;
mod process_types;
mod process_wait;
pub use process_authority::ProcessAuthority;
pub use process_prelude::inject;
