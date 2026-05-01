//! Guarded TUI drawing.

use ratatui::{Terminal, backend::CrosstermBackend, layout::Size};

use crate::session::Session;
use crate::tui::{app::state::App, ui::main::ui};

const MAX_RENDER_CELLS: u32 = 500_000;
const MAX_RENDER_DIMENSION: u16 = 1_000;

/// Draw one TUI frame when the reported terminal size is sane.
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &Session,
) -> anyhow::Result<()> {
    let size = terminal.size()?;
    if !safe_size(size) {
        tracing::warn!(
            width = size.width,
            height = size.height,
            cells = size.width as u32 * size.height as u32,
            "skipping TUI draw because terminal reported an invalid size"
        );
        return Ok(());
    }
    terminal.draw(|f| ui(f, app, session))?;
    Ok(())
}

fn safe_size(size: Size) -> bool {
    let cells = size.width as u32 * size.height as u32;
    size.width > 0
        && size.height > 0
        && size.width <= MAX_RENDER_DIMENSION
        && size.height <= MAX_RENDER_DIMENSION
        && cells <= MAX_RENDER_CELLS
}

#[cfg(test)]
mod tests {
    use super::safe_size;
    use ratatui::layout::Size;

    #[test]
    fn rejects_absurd_terminal_sizes() {
        assert!(safe_size(Size::new(120, 40)));
        assert!(!safe_size(Size::new(0, 40)));
        assert!(!safe_size(Size::new(u16::MAX, u16::MAX)));
    }
}
