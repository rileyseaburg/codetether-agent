//! Board glyphs and paddle/star placement.
use crate::{InterludeState, LANES, ROWS};
use ratatui::text::Line;

pub(super) fn board(game: &InterludeState) -> Vec<Line<'static>> {
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
