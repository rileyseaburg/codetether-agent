//! User password login and the local saved-credential cache.
//!
//! [login_with_password] preserves a caller-supplied application version in the
//! HTTP User-Agent. [write_saved_credentials] stores its response; [load_saved_credentials]
//! reads unexpired records from the platform configuration directory.
mod load;
mod login;
mod path;
mod record;
mod reply;
mod save;
pub use load::load_saved_credentials;
pub use login::login_with_password;
pub use record::SavedCredentials;
pub use reply::LoginResponsePayload;
pub use save::write_saved_credentials;
