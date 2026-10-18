//! Fit one span while preserving style and consuming input.
use crate::{fit::take_fit, wrap_state::WrapState};
use ratatui::{
    style::Style,
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;
pub(crate) fn append(state: &mut WrapState, style: Style, mut text: String, width: usize) {
    while !text.is_empty() {
        let remaining = width.saturating_sub(state.cur_w);
        if remaining == 0 {
            state.out.push(Line::from(std::mem::take(&mut state.cur)));
            state.cur_w = 0;
            continue; // next iter has remaining == width > 0
        }
        let (taken, rest) = take_fit(&text, remaining, state.cur_w == 0);
        if taken.is_empty() {
            if state.cur_w > 0 {
                // Mid-row and nothing fit: flush and retry at col 0,
                // where the full width is available.
                state.out.push(Line::from(std::mem::take(&mut state.cur)));
                state.cur_w = 0;
                continue;
            }
            // At col 0 with nothing taken: `rest` is the remainder
            // `take_fit` could not consume (e.g. leading whitespace it
            // trimmed). Adopt it directly so `text` strictly shrinks;
            // if it didn't shrink, the remainder is unrenderable and we
            // stop to keep the loop bounded.
            if rest.len() >= text.len() {
                break;
            }
            text = rest;
            continue;
        }
        state.cur_w += UnicodeWidthStr::width(taken.as_str());
        state.cur.push(Span::styled(taken, style));
        text = rest;
        if !text.is_empty() {
            state.out.push(Line::from(std::mem::take(&mut state.cur)));
            state.cur_w = 0;
        }
    }
}
