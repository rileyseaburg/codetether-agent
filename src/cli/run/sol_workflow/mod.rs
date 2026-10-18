//! Sol workflow bound to native session and validation services.
mod session;
pub(super) type NativeHost = codetether_native_cli::sol::Host<
    crate::session::Session,
    crate::session::helper::validation::ApplicationValidation,
>;
