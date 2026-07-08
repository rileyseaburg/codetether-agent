//! In-flight elapsed-time badge for streaming previews.
//!
//! The badge heats up as the request ages: cool green under 3 s,
//! blending toward amber by 10 s and hot red past 30 s — you can feel
//! a slow request without reading the number.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::AppState;
use crate::tui::ui::gradient::{lerp_rgb, rgb_supported};

const COOL: (u8, u8, u8) = (0, 255, 128);
const WARM: (u8, u8, u8) = (255, 200, 0);
const HOT: (u8, u8, u8) = (255, 60, 60);

/// Build an elapsed-time badge span for the streaming header.
pub fn elapsed_badge(state: &AppState) -> Span<'static> {
    let ms = state.current_request_elapsed_ms();
    let label = ms.map(elapsed_label).unwrap_or_default();
    let color = ms.map_or(Color::Yellow, heat_color);
    Span::styled(
        label,
        Style::default().fg(color).add_modifier(Modifier::DIM),
    )
}

fn heat_color(ms: u64) -> Color {
    if !rgb_supported() {
        return Color::Yellow;
    }
    let secs = ms as f32 / 1_000.0;
    if secs <= 10.0 {
        lerp_rgb(COOL, WARM, (secs / 10.0).clamp(0.0, 1.0))
    } else {
        lerp_rgb(WARM, HOT, ((secs - 10.0) / 20.0).clamp(0.0, 1.0))
    }
}

fn elapsed_label(ms: u64) -> String {
    if ms >= 1_000 {
        format!(" {:.1}s", ms as f64 / 1_000.0)
    } else {
        format!(" {ms}ms")
    }
}
