use super::{Action, App};
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::Insert(c) => {
            app.state.insert_char(*c);
        }
        Action::DeleteBackspace => {
            app.state.delete_backspace();
        }
        _ => return false,
    }
    true
}
