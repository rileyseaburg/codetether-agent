//! Select and scroll agent details, or render the empty state.
use crate::{RegistryView, detail_lines::detail_lines};
use codetether_a2a_core::AgentCard;
use ratatui::widgets::{Paragraph, Wrap};

pub(crate) fn detail_widget(
    view: &RegistryView<'_>,
    cards: &[AgentCard],
) -> Option<Paragraph<'static>> {
    let idx = view.selected.min(cards.len().saturating_sub(1));
    let card = cards.get(idx)?;
    Some(
        Paragraph::new(detail_lines(card))
            .wrap(Wrap { trim: false })
            .scroll((view.scroll as u16, 0)),
    )
}

pub(crate) fn empty_widget<'a>() -> Paragraph<'a> {
    Paragraph::new("No agent cards registered.\nConnect to an A2A server to see cards here.")
        .wrap(Wrap { trim: false })
}
