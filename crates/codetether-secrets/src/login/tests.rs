//! Local URL/credential persistence and namespace-boundary regression tests.

use super::*;

#[test]
fn vault_urls_require_secure_unambiguous_endpoints() {
    assert_eq!(
        normalize("https://vault.example/").unwrap(),
        "https://vault.example"
    );
    assert!(normalize("http://127.0.0.1:8200").is_ok());
    for url in [
        "http://vault.example",
        "https://user:secret@vault.example",
        "https://vault.example?token=x",
        "https://vault.example#x",
    ] {
        assert!(normalize(url).is_err());
    }
}

#[test]
fn administrator_roles_are_not_login_choices() {
    for role in ["admin", "superadmin", "root", "ADMIN"] {
        assert!(http::auth_path("oidc", role).is_err());
    }
    assert_eq!(
        http::auth_path("oidc", "codetether").unwrap(),
        "auth/oidc/login"
    );
    assert!(http::auth_path("../oidc", "codetether").is_err());
}
