//! Shell discovery and noninteractive subprocess preparation.
//!
//! [resolve] returns a [Shell] invocation; [configure] prepares its command
//! with piped output, disabled interactive authentication and process ownership.
//! [apply_vault_token] gives direct commands the current rotated Vault token.
mod configure;
mod shell;
mod vault_token;
pub use configure::configure;
pub use shell::{Shell, resolve};
pub use vault_token::apply_vault_token;
