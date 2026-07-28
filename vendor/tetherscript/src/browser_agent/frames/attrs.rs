use crate::browser::Element;

const BLANK_FRAME_URL: &str = "about:blank";

pub(super) fn frame_name(element: &Element) -> String {
    element.attrs.get("name").cloned().unwrap_or_default()
}

pub(super) fn frame_url(element: &Element) -> String {
    element
        .attrs
        .get("src")
        .cloned()
        .unwrap_or_else(|| BLANK_FRAME_URL.to_string())
}
