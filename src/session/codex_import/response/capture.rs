use crate::provider::Role;

pub(super) fn capture_user_text(role: &Role, text: &str, first_user_text: &mut Option<String>) {
    if role == &Role::User
        && first_user_text.is_none()
        && crate::session::title::is_title_candidate(text)
    {
        *first_user_text = Some(text.to_string());
    }
}
