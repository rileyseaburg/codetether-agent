//! Question and continuation interception before tool feedback or steering.
use crate::Journal;
/// Intercept explicit question/continuation input using the current session view.
/// # Arguments
/// `app` supplies composer state, journal access and cancellation callbacks.
/// # Returns
/// True if the workflow consumed the input; false to continue ordinary routing.
/// # Examples
/// ```no_run
/// async fn input<H: codetether_goal_answer::Input>(host: &mut H) {
///     codetether_goal_answer::intercept(host).await;
/// }
/// ```
pub async fn intercept<H: crate::Input>(app: &mut H) -> bool {
    let input = app.input().trim().to_string();
    let id = app.session_id();
    match super::agentic::release_ask(app, &id).await {
        Ok(true) => return false,
        Ok(false) => {}
        Err(error) => {
            app.status(format!("Cannot safely resume goal: {error}"));
            app.redraw();
            return true;
        }
    }
    if super::continuation::intercept(app).await {
        return true;
    }
    let question = super::question::parse(&input).or_else(|| {
        H::Journal::held(&app.session_id())
            .then(|| super::question::followup(&input))
            .flatten()
    });
    let Some(question) = question else {
        return false;
    };
    let id = app.session_id();
    super::review::begin(app, &id, question, &input).await
}
