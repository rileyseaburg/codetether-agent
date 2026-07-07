//! Gradient welcome banner shown when the chat is empty.
//!
//! Replaces the old single dim placeholder line with a neon-gradient
//! wordmark, a fading rule, and quick-start hints. Falls back to plain
//! cyan on terminals without truecolor support.

use ratatui::style::Stylize;
use ratatui::text::Line;

use super::gradient::{NEON_CYAN, NEON_MAGENTA, rgb_supported};
use super::gradient_rule::gradient_rule;
use super::gradient_sweep::sweep_spans;

const WORDMARK: &str = "  ⬡  C O D E T E T H E R";
const RULE_WIDTH: usize = 30;

/// Push the neon welcome banner into `lines`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::banner::push_welcome_banner;
/// let mut lines: Vec<ratatui::text::Line<'static>> = Vec::new();
/// push_welcome_banner(&mut lines);
/// assert!(lines.len() >= 5);
/// ```
pub fn push_welcome_banner(lines: &mut Vec<Line<'static>>) {
    lines.push(Line::default());
    lines.push(wordmark_line());
    lines.push(rule_line());
    lines.push(Line::from("  Type a prompt and press Enter to begin".dim()));
    lines.push(Line::from(
        "  /help commands · /model pick a model · Tab switch agent".dim(),
    ));
}

fn wordmark_line() -> Line<'static> {
    if rgb_supported() {
        // Animated sweep: the gradient band travels through the wordmark.
        Line::from(sweep_spans(WORDMARK, NEON_CYAN, NEON_MAGENTA, true, 4_000))
    } else {
        Line::from(WORDMARK.cyan().bold())
    }
}

fn rule_line() -> Line<'static> {
    if rgb_supported() {
        gradient_rule("━", RULE_WIDTH, NEON_CYAN, (40, 20, 60))
    } else {
        Line::from("━".repeat(RULE_WIDTH).dark_gray())
    }
}
