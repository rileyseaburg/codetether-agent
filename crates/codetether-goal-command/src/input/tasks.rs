//! Display work items separately from the session objective.
pub(super) async fn show<H: super::Host>(app: &mut H, rest: &str) {
    let session_id = app.session_id();
    if !matches!(rest.trim(), "" | "list") {
        app.status("Usage: /tasks [list] — /goal manages the objective".into());
        return;
    }
    match app.tasks(&session_id) {
        Ok(state) => {
            app.system(codetether_goal_core::task_block(&state));
            app.status("Session tasks (work items); use /goal for the objective".into());
            app.scroll();
        }
        Err(error) => app.status(format!("/tasks: {error}")),
    }
}
