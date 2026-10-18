//! Title lifecycle with native state writes supplied by the session adapter.
use crate::{TitleSession, display, extract};
use anyhow::Result;
/// Generate a heuristic title only when no title is stored.
/// # Arguments
/// `session` supplies the native title, transcript and write callbacks.
/// # Returns
/// Unit after the title/timestamp callbacks complete.
/// # Errors
/// Retains the existing fallible lifecycle API; the current heuristic has no error return.
/// # Examples
/// ```rust,no_run
/// use codetether_session_title::{TitleSession, generate};
/// # async fn example<S: TitleSession>(session: &mut S) -> anyhow::Result<()> {
/// generate(session).await?;
/// # Ok(()) }
/// ```
pub async fn generate<S: TitleSession>(session: &mut S) -> Result<()> {
    if session.title().is_some() {
        return Ok(());
    }
    set_from_first_user_message(session);
    Ok(())
}
/// Replace any stored title with the first eligible user-message title.
/// # Arguments
/// `session` supplies the native title, transcript and write callbacks.
/// # Returns
/// Unit after the title/timestamp callbacks complete.
/// # Errors
/// Retains the existing fallible lifecycle API; the current heuristic has no error return.
/// # Examples
/// ```rust,no_run
/// use codetether_session_title::{TitleSession, regenerate};
/// # async fn example<S: TitleSession>(session: &mut S) -> anyhow::Result<()> {
/// regenerate(session).await?;
/// # Ok(()) }
/// ```
pub async fn regenerate<S: TitleSession>(session: &mut S) -> Result<()> {
    set_from_first_user_message(session);
    Ok(())
}
fn set_from_first_user_message<S: TitleSession>(session: &mut S) {
    if let Some(title) = extract::title_from_first_user_message(session.messages()) {
        set(session, title);
    }
}
/// Normalize and store a candidate title, then update its timestamp.
/// # Arguments
/// `session` supplies the native field callbacks; `title` is the candidate text.
/// # Returns
/// Unit after updating the native fields.
/// # Examples
/// ```rust
/// # fn example<S: codetether_session_title::TitleSession>(session: &mut S) {
/// codetether_session_title::set(session, "Review storage");
/// # }
/// ```
pub fn set<S: TitleSession>(session: &mut S, title: impl Into<String>) {
    session.store_title(display::normalize_title(&title.into()));
    session.touch();
}
/// Clear the stored title and update its timestamp.
/// # Arguments
/// `session` supplies the native field callbacks.
/// # Returns
/// Unit after updating the native fields.
/// # Examples
/// ```rust
/// # fn example<S: codetether_session_title::TitleSession>(session: &mut S) {
/// codetether_session_title::clear(session);
/// # }
/// ```
pub fn clear<S: TitleSession>(session: &mut S) {
    session.store_title(None);
    session.touch();
}
/// Update the native timestamp and optionally regenerate the title.
/// # Arguments
/// `session` supplies the native title, transcript and write callbacks.
/// # Returns
/// Unit after the title/timestamp callbacks complete.
/// # Errors
/// Retains the existing fallible lifecycle API; the current heuristic has no error return.
/// # Examples
/// ```rust,no_run
/// use codetether_session_title::{TitleSession, context_change};
/// # async fn example<S: TitleSession>(session: &mut S) -> anyhow::Result<()> {
/// context_change(session, true).await?;
/// # Ok(()) }
/// ```
pub async fn context_change<S: TitleSession>(
    session: &mut S,
    regenerate_title: bool,
) -> Result<()> {
    session.touch();
    if regenerate_title {
        regenerate(session).await?;
    }
    Ok(())
}
