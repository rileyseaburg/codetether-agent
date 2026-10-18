//! Static-token identity and cached admin opt-in.
use crate::PolicyUser;

pub(super) fn static_user(roles: Vec<String>, auth_source: &str) -> PolicyUser {
    PolicyUser {
        user_id: "bearer-token-user".to_string(),
        roles,
        tenant_id: None,
        scopes: Vec::new(),
        auth_source: auth_source.to_string(),
    }
}

pub(super) fn static_token_admin_enabled() -> bool {
    static ENABLED: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
        codetether_util::environment::env_bool("CODETETHER_STATIC_TOKEN_ADMIN", false)
    });
    *ENABLED
}
