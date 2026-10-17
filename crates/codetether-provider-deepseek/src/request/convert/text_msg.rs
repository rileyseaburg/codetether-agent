//! text msg conversion.
use super::*;

pub(super) fn text_msg(m: &Message) -> Value {
    let role = match m.role {
        Role::System | Role::Developer => "system",
        _ => "user",
    };
    json!({"role": role, "content": convert_helpers::collect_text(m)})
}
