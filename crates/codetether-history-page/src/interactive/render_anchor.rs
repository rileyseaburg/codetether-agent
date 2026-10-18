//! Preserve logical viewport position when older messages are prepended.
/// Restore the logical viewport position after a page prepend.
///
/// # Arguments
/// * `app` — Chat view and pager state.
/// * `total_lines` — Current rendered transcript size.
///
/// # Returns
/// Updates the viewport and requests more history if rewind remains.
///
/// # Examples
/// ```no_run
/// # fn update<S: codetether_history_page::HistorySource + 'static>(
/// #     view: &mut impl codetether_history_page::interactive::View<S>) {
/// codetether_history_page::interactive::apply_pending_history_anchor(view, 100);
/// # }
/// ```
pub fn apply_pending_history_anchor<S: crate::HistorySource + 'static>(
    app: &mut impl super::View<S>,
    total_lines: usize,
) {
    let Some((inserted, old_scroll, rewind)) = app.pager().take_anchor(total_lines) else {
        return;
    };
    let anchored = inserted.saturating_add(old_scroll);
    app.set_scroll(anchored.saturating_sub(rewind));
    app.set_auto_follow(false);
    let remaining = rewind.saturating_sub(anchored);
    if remaining > 0 {
        let visible = app.messages().len();
        app.pager().request_older(remaining, visible);
    }
}
