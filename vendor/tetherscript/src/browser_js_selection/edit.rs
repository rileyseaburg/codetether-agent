use super::*;

pub(super) fn replace(handle: &DomHandle, insert: &str) -> Option<String> {
    let Node::Element(element) = handle.node()? else {
        return None;
    };
    if !is_contenteditable(&element) {
        return None;
    }
    let current = text_content_raw(&Node::Element(element.clone()));
    let next = replace_selected_text(handle, &element, &current, insert)
        .unwrap_or_else(|| format!("{current}{insert}"));
    edit_set::content(handle, next.clone());
    let cursor = next.chars().count();
    registry::set_selection(Some(state::RangeState::collapsed(handle, cursor)));
    Some(next)
}

fn replace_selected_text(
    handle: &DomHandle,
    element: &Element,
    current: &str,
    insert: &str,
) -> Option<String> {
    let selection = registry::selection()?;
    if selection.is_collapsed() {
        return None;
    }
    if selection.start.handle.path != handle.path || selection.end.handle.path != handle.path {
        return (text::range_text(&selection) == current).then(|| insert.to_string());
    }
    let start = text::child_char_offset(element, selection.start.offset);
    let end = text::child_char_offset(element, selection.end.offset);
    Some(format!(
        "{}{}{}",
        text::char_slice(current, 0, start),
        insert,
        text::char_slice(current, end, current.chars().count())
    ))
}
