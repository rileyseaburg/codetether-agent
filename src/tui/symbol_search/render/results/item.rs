use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::tui::symbol_search::SymbolEntry;

pub fn render(symbol: &SymbolEntry) -> ListItem<'_> {
    let location = symbol
        .line
        .map(|line| format!(":{line}"))
        .unwrap_or_default();
    ListItem::new(Line::from(vec![
        Span::styled(
            format!(" {:8} ", symbol.kind),
            Style::default().fg(kind_color(&symbol.kind)),
        ),
        Span::from(symbol.name.as_str()).white().bold(),
        Span::from(format!(" → {}{location}", symbol.path.display())).dark_gray(),
    ]))
}

fn kind_color(kind: &str) -> Color {
    match kind {
        "Function" | "Method" => Color::Yellow,
        "Struct" | "Class" | "Enum" | "Interface" => Color::Cyan,
        "Module" | "Namespace" => Color::Magenta,
        "Constant" | "Field" | "Property" => Color::Green,
        _ => Color::White,
    }
}
