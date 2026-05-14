//! Mutate retained live agent detail buffers.

pub fn push<T>(items: &mut Vec<T>, item: T) {
    crate::tui::agent_detail_retain::push(items, item);
}

pub fn output(slot: &mut Option<String>, text: &str, label: &str) {
    *slot = Some(crate::tui::agent_detail_retain::output(text, label));
}
