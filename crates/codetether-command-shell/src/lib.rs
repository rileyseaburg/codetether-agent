//! Shell discovery and noninteractive subprocess preparation.
//!
//! [resolve] returns a [Shell] invocation; [configure] prepares its command
//! with piped output, disabled interactive authentication and process ownership.
mod configure;
mod shell;
pub use configure::configure;
pub use shell::{Shell, resolve};
