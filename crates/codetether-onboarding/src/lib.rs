//! Interactive account onboarding for command-line applications.
//!
//! [ensure_authenticated] reuses saved credentials or offers registration,
//! login and local-only operation. [register_and_login] and [login_interactive]
//! are separate entry points using the shared authentication client/cache.
mod entry;
mod login;
mod prompt;
mod register;
pub use entry::ensure_authenticated;
pub use login::login_interactive;
pub use prompt::prompt_line;
pub use register::register_and_login;
/// Default service endpoint used by interactive onboarding.
pub const DEFAULT_SERVER: &str = "https://api.codetether.run";
