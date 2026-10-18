//! Context percentage labels and severity colors.
use ratatui::style::{Color, Modifier, Style};

pub(crate) fn label_for(pct: f64) -> String {
    let icon = if pct >= 90.0 {
        "🛑"
    } else if pct >= 75.0 {
        "⚠️"
    } else {
        "🧠"
    };
    format!("{icon} Context: {pct:.0}%")
}

pub(crate) fn style_for(pct: f64) -> Style {
    let color = if pct >= 90.0 {
        Color::Red
    } else if pct >= 75.0 {
        Color::Yellow
    } else if pct >= 50.0 {
        Color::Cyan
    } else {
        Color::Green
    };
    let base = Style::default().fg(color);
    if pct >= 75.0 {
        base.add_modifier(Modifier::BOLD)
    } else {
        base
    }
}
