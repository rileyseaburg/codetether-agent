use super::*;

pub(super) fn range_text(state: &state::RangeState) -> String {
    if !Rc::ptr_eq(&state.start.handle.root, &state.end.handle.root) {
        return String::new();
    }
    if state.start.handle.path != state.end.handle.path {
        return String::new();
    }
    text_inside(&state.start.handle, state.start.offset, state.end.offset)
}

pub(super) fn text_inside(handle: &DomHandle, start: usize, end: usize) -> String {
    let end = end.max(start);
    match handle.node() {
        Some(Node::Text(text)) => char_slice(&text, start, end),
        Some(Node::Element(el)) if el.tag == "input" => {
            char_slice(&handle.input_value(), start, end)
        }
        Some(Node::Element(el)) => child_text(&el, start, end),
        None => String::new(),
    }
}

pub(super) fn char_slice(text: &str, start: usize, end: usize) -> String {
    text.chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

pub(super) fn child_char_offset(element: &Element, offset: usize) -> usize {
    element
        .children
        .iter()
        .take(offset)
        .map(text_content_raw)
        .map(|text| text.chars().count())
        .sum()
}

fn child_text(element: &Element, start: usize, end: usize) -> String {
    element
        .children
        .iter()
        .skip(start)
        .take(end.saturating_sub(start))
        .map(text_content_raw)
        .collect::<Vec<_>>()
        .join("")
}
