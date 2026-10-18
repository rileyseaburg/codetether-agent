//! Explicit goal execution and input bookkeeping.
pub(super) async fn run<H: super::Host>(app: &mut H, input: &str) {
    let id = app.session_id();
    let changed = if input == "/go" {
        app.goal(&id, "execute").await
    } else {
        app.status(
            "/go takes no arguments; it executes the current goal without a budget cap".into(),
        );
        false
    };
    app.history(input.to_string());
    app.clear();
    if changed {
        app.restart(&id);
    }
    app.redraw();
}
