//! Settings setting value span.
use ratatui::{style::Stylize, text::Span};

pub(super) fn setting_value_span(enabled: bool) -> Span<'static> {
    if enabled {
        "ON".green().into()
    } else {
        "OFF".yellow().into()
    }
}
