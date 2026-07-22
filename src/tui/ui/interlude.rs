//! Centered Tether Catch play-break overlay.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::tui::app::state::{
    App,
    interlude::{InterludeState, LANES, ROWS},
};

#[path = "interlude_area.rs"]
mod area;

pub fn render_if_active(f: &mut Frame, area: Rect, app: &mut App) {
    if !app.state.processing {
        return;
    }
    let Some(game) = app.state.interlude.as_mut() else {
        return;
    };
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

fn board(game: &InterludeState) -> Vec<Line<'static>> {
    let (star_lane, star_row) = game.star();
    let mut lines = vec![Line::from(" Catch ✦ with ← → or A/D"), Line::from("")];
    for row in 0..ROWS {
        let marker = (star_row == row).then_some((star_lane, " ✦ "));
        lines.push(Line::from(cells(marker)));
    }
    lines.push(Line::from(cells(Some((game.lane(), " ▲ ")))));
    lines
}

fn cells(marker: Option<(u8, &str)>) -> String {
    (0..LANES)
        .map(|lane| match marker {
            Some((target, glyph)) if lane == target => glyph,
            _ => " · ",
        })
        .collect()
}
