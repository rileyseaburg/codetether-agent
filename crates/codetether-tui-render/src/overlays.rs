//! Global overlay order from symbol search through questions.
use crate::Overlays;
use ratatui::Frame;
pub(super) fn render(frame: &mut Frame, host: &mut impl Overlays) {
    if host.symbols_active() {
        host.symbols(frame);
    }
    host.goal_prompt(frame);
    host.fuzzy_find(frame);
    host.interlude(frame);
    host.watchdog(frame);
    host.answer_review(frame);
    host.question(frame);
}
