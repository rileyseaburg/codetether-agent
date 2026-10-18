//! Age-colored `ListItem` builder for the session picker.

use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::ListItem,
};

use crate::age_color::age_color;
use crate::row::session_row_summary;
use codetether_session_metadata::SessionSummary;
use codetether_theme::gradient::rgb_supported;

/// Build an age-colored [`ListItem`] for one session row.
///
/// # Arguments
/// * `session` - Session metadata to display.
/// * `is_active` - Whether this is the loaded session.
///
/// # Returns
/// An owned row with age-dependent foreground color.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_session_picker::session_list_item;
/// ```
pub fn session_list_item(session: &SessionSummary, is_active: bool) -> ListItem<'static> {
    let text = session_row_summary(session, is_active);
    let color = age_color(session.updated_at, rgb_supported());
    ListItem::new(Line::from(Span::styled(text, Style::default().fg(color))))
}
