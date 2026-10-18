//! Terminal widgets for an active play break.
use crate::{InterludeState, area, board::board};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
/// Advance and draw an active game inside the supplied rectangle.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `area` - Area in which the popup is centered.
/// * `game` - Active game to advance and display.
///
/// # Returns
/// Draws a centered board after updating its clock.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, game: &mut codetether_interlude::InterludeState) {
/// let area = f.area();
/// codetether_interlude::render(f, area, game);
/// # }
/// ```
pub fn render(f: &mut Frame, area: Rect, game: &mut InterludeState) {
    game.advance();
    let popup = self::area::popup(area);
    let title = format!(" Tether Catch · score {} ", game.score());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);
    f.render_widget(Clear, popup);
    f.render_widget(Paragraph::new(board(game)).block(block), popup);
}
