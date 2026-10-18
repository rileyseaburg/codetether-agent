//! Convert an ask on a held goal into an ordinary continuation request.
use crate::Journal;
pub(super) async fn release_ask<H: crate::Input>(
    app: &mut H,
    session_id: &str,
) -> anyhow::Result<bool> {
    if !H::Journal::held(session_id) {
        return Ok(false);
    }
    let input = app.input().trim();
    let Some(suffix) = input.strip_prefix("/ask") else {
        return Ok(false);
    };
    if suffix.is_empty() || !suffix.starts_with(char::is_whitespace) {
        return Ok(false);
    }
    let request = suffix.trim();
    if request.is_empty() {
        return Ok(false);
    }
    let request = request.to_string();
    if !super::release::release::<H::Journal>(session_id).await? {
        return Ok(false);
    }
    app.rewrite(request);
    app.answer_no();
    Ok(true)
}
