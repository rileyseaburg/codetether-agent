//! Device authorization instructions and browser opening.
use super::oidc_types;

/// Print the verification URL/code and attempt to open the browser.
pub(super) fn prompt_user(auth: &oidc_types::DeviceAuthorization) {
    let url = auth
        .verification_uri_complete
        .as_deref()
        .unwrap_or(&auth.verification_uri);
    eprintln!("To authorize, open: {url}");
    eprintln!("Then enter the code: {}", auth.user_code);
    let _ = open::that_detached(url);
}
