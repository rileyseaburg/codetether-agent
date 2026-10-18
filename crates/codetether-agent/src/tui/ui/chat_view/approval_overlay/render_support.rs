//! Mocked local terminal rendering with optional persistent text snapshots.

use crate::tui::app::state::App;
use ratatui::{Terminal, backend::TestBackend};

pub(in crate::tui::ui::chat_view::approval_overlay) fn draw(
    app: &mut App,
    width: u16,
    height: u16,
    label: &str,
) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal
        .draw(|frame| {
            let area = frame.area();
            super::super::render(frame, app, area);
        })
        .unwrap();
    let buffer = terminal.backend().buffer();
    let rows: Vec<String> = buffer
        .content
        .chunks(usize::from(width))
        .map(|row| row.iter().map(|cell| cell.symbol()).collect())
        .collect();
    if let Some(path) = std::env::var_os("CODETETHER_APPROVAL_EVIDENCE_DIR") {
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(
            path.join(format!("{label}-{width}x{height}.txt")),
            rows.join("\n"),
        )
        .unwrap();
    }
    rows.join("")
}
