//! Begin a durable question hold and present its result.
use crate::Journal;
pub(super) async fn begin<H: crate::Input>(
    app: &mut H,
    id: &str,
    question: &str,
    input: &str,
) -> bool {
    match H::Journal::begin(id, question).await {
        Ok(false) => false,
        Ok(true) => {
            app.cancel();
            app.history(input.to_string());
            app.clear();
            app.answer_no();
            app.status("Goal paused — answering your question before continuing".into());
            app.redraw();
            true
        }
        Err(error) => {
            app.cancel();
            app.status(format!("Cannot safely pause goal: {error}"));
            true
        }
    }
}
