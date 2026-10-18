//! Store-and-take ownership transfer after a rebuilt frame.
use super::Host;
use crate::DrawnLines;
use ratatui::text::Line;
pub(super) fn store_and_take(
    host: &mut impl Host,
    lines: Vec<Line<'static>>,
    max_width: usize,
    frozen_len: usize,
) -> DrawnLines {
    let cacheable = !host.cache_inputs().pending_tool;
    let inputs = host.cache_inputs();
    host.cache()
        .store_message_lines_with_frozen(lines, max_width, frozen_len, inputs);
    let lines = host.cache().take_cached_message_lines();
    if cacheable {
        DrawnLines::from_cache(lines)
    } else {
        DrawnLines::from_rebuild(lines)
    }
}
