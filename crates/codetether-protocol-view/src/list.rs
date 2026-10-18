//! Agent card list titles and widgets.
use crate::list_row::list_item;
use codetether_a2a_core::AgentCard;
use ratatui::widgets::{Block, Borders, List};

pub(crate) fn list_widget(cards: &[AgentCard]) -> List<'static> {
    let items = cards.iter().map(list_item).collect::<Vec<_>>();
    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(list_title(cards)),
    )
}

pub(crate) fn list_title(cards: &[AgentCard]) -> String {
    format!("Agent Cards ({})", cards.len())
}
