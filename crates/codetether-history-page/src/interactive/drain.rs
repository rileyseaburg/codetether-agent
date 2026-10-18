//! Merge completed history pages into the chronological chat buffer.

use super::View;
use crate::HistorySource;

/// Apply one current-generation page result and report whether one was found.
///
/// # Arguments
/// * `app` — Chat view and pager state.
///
/// # Returns
/// Whether a page or error was consumed.
///
/// # Examples
/// ```no_run
/// # fn update<S: codetether_history_page::HistorySource + 'static>(
/// #     view: &mut impl codetether_history_page::interactive::View<S>) {
/// let _ = codetether_history_page::interactive::drain(view);
/// # }
/// ```
pub fn drain<S: HistorySource + 'static>(app: &mut impl View<S>) -> bool {
    let Some(result) = app.pager().take_result() else {
        return false;
    };
    match result {
        Ok(page) => apply(app, page),
        Err(error) => {
            app.pager().fail();
            app.set_status(format!("Failed to load older history: {error}"));
        }
    }
    true
}

fn apply<S: HistorySource + 'static>(app: &mut impl View<S>, page: super::Page) {
    let follow_latest = app.follow_latest();
    let old_scroll = app.manual_scroll();
    let old_lines = {
        let cached = app.cached_lines();
        let max_scroll = app.last_max_scroll();
        cached.max(app.pager().known_total_lines(max_scroll))
    };
    let mut older =
        codetether_chat_transcript::provider_messages_to_chat_messages(page.messages.iter());
    let visible_added = older.len();
    older.append(app.messages());
    *app.messages() = older;
    app.clear_cache();
    app.pager()
        .accept(&page, old_lines, old_scroll, follow_latest);
    app.set_status(if page.exhausted {
        "Loaded the beginning of this session".to_string()
    } else {
        format!("Loaded {visible_added} older messages")
    });
}
