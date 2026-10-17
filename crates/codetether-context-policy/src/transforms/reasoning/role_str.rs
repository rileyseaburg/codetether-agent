//! role str conversion.
use super::*;

pub(super) fn role_str(role: Role) -> &'static str {
    match role {
        Role::Assistant => "assistant",
        Role::User => "user",
        Role::Tool => "tool",
        Role::System | Role::Developer => "system",
    }
}
