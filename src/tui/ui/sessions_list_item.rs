//! Age-colored `ListItem` builder for the session picker.

use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::ListItem,
};

use crate::session::SessionSummary;
use crate::tui::ui::gradient::rgb_supported;
use crate::tui::ui::sessions_age_color::age_color;
use crate::tui::ui::sessions_row::session_row_summary;

/// Build an age-colored [`ListItem`] for one session row.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::sessions_list_item::session_list_item;
/// ```
pub fn session_list_item(session: &SessionSummary, is_active: bool) -> ListItem<'static> {
    let text = session_row_summary(session, is_active);
    let color = age_color(session.updated_at, rgb_supported());
    ListItem::new(Line::from(Span::styled(text, Style::default().fg(color))))
}
