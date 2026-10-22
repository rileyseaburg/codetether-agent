//! Bounded source references; full user evidence remains in the durable checkpoint.
use crate::{UserEvidence, bounded::excerpt};
pub(super) fn render(sources: &[UserEvidence]) -> String {
    let mut budget = 10_000usize;
    let mut selected = Vec::new();
    // Preserve the first request and then prioritize the latest corrections.
    let order = (0..sources.len().min(1)).chain((1..sources.len()).rev());
    for index in order {
        if budget < 300 {
            break;
        }
        let source = &sources[index];
        let item = format!(
            "[user turn {}]\n{}\n",
            source.source_turn,
            excerpt(&source.text, budget.min(4096).saturating_sub(80))
        );
        budget = budget.saturating_sub(item.len());
        selected.push((index, item));
    }
    let omitted = sources.len().saturating_sub(selected.len());
    selected.sort_by_key(|(index, _)| *index);
    let mut text: String = selected.into_iter().map(|(_, item)| item).collect();
    if omitted > 0 || sources.iter().any(|source| source.text.len() > 4016) {
        let start = sources.first().map_or(0, |source| source.source_turn);
        let end = sources.last().map_or(0, |source| source.source_turn);
        text.push_str(&format!("[Bounded preview: {omitted} user messages omitted, and long messages may be excerpts. All complete text is retained in the durable checkpoint. Recover missing source turns with context_browse before actions depending on them; omitted text is not permission.]\n"));
        text.push_str(&format!(
            "[Full user-source span: original turns {start} through {end}.]\n"
        ));
    }
    text
}
