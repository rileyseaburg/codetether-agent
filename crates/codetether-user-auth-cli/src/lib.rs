//! User account registration and password-login command flows.
//!
//! [register] and [login] accept caller version metadata and use the shared
//! user-auth client/cache. Prompts, registration transport and output each
//! have their own module; [rpassword_prompt] is shared with native onboarding.
mod email;
mod login;
mod login_args;
mod login_report;
mod password;
mod register;
mod register_args;
mod register_report;
mod register_request;
mod register_response;
mod registration_password;
pub use login::execute as login;
pub use login_args::LoginAuthArgs;
pub use password::rpassword_prompt;
pub use register::execute as register;
pub use register_args::RegisterAuthArgs;
