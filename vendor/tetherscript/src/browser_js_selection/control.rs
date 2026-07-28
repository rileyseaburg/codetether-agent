use super::*;

pub(super) fn active_control_text() -> Option<String> {
    let handle = FOCUSED_ELEMENT.with(|focused| {
        focused
            .borrow()
            .as_ref()
            .and_then(|key| handle_by_event_key(key))
    })?;
    let Node::Element(element) = handle.node()? else {
        return None;
    };
    if !matches!(element.tag.as_str(), "input" | "textarea") {
        return None;
    }
    let (mut start, mut end) = selection_for_handle(&handle);
    let value = handle.input_value();
    let len = value.chars().count();
    start = start.min(len);
    end = end.min(len);
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }
    Some(text::char_slice(&value, start, end))
}
