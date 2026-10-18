//! Recognize whole-input human continuation commands.
/// Whether input is one of the existing explicit continuation commands.
/// # Arguments
/// `input` is the complete composer text.
/// # Returns
/// True for a supported command, ignoring surrounding space and ASCII case.
/// # Examples
/// ```
/// assert!(codetether_goal_answer::continuation::requested(" CONTINUE "));
/// assert!(!codetether_goal_answer::continuation::requested("continue?"));
/// ```
pub fn requested(input: &str) -> bool {
    matches!(
        input.trim().to_ascii_lowercase().as_str(),
        "continue" | "coninue" | "resume" | "/continue"
    )
}

pub(super) async fn intercept<H: crate::Input>(app: &mut H) -> bool {
    if !requested(app.input()) {
        return false;
    }
    let id = app.session_id();
    match super::release::release::<H::Journal>(&id).await {
        Ok(false) => return false,
        Ok(true) => {
            app.history(app.input().trim().to_string());
            app.clear();
            app.answer_no();
            app.restart(&id);
            app.status("Goal continuation requested".into());
        }
        Err(error) => app.status(format!("Cannot safely resume goal: {error}")),
    }
    app.redraw();
    true
}
