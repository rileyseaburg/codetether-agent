//! Display login completion and credential location.
use codetether_user_auth::LoginResponsePayload;
use std::path::Path;

pub(super) fn print(login: &LoginResponsePayload, email: &str, cred_path: &Path) {
    let user_email = login
        .user
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or(email);

    println!("Logged in as {} (expires {})", user_email, login.expires_at);
    println!("Credentials saved to {}", cred_path.display());
    println!("\nThe CLI will automatically use these credentials for `codetether worker`.");
}
