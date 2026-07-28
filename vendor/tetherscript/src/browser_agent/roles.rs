//! HTML-to-ARIA role mapping for locators.

use crate::browser::Element;

pub(crate) fn role_of(element: &Element) -> String {
    if let Some(role) = element.attrs.get("role") {
        if !role.trim().is_empty() {
            return role.to_ascii_lowercase();
        }
    }
    implicit_role(element).unwrap_or("generic").to_string()
}

fn implicit_role(element: &Element) -> Option<&'static str> {
    Some(match element.tag.as_str() {
        "a" if element.attrs.contains_key("href") => "link",
        "button" => "button",
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "heading",
        "img" => "img",
        "input" => input_role(element),
        "li" => "listitem",
        "main" => "main",
        "nav" => "navigation",
        "ol" | "ul" => "list",
        "option" => "option",
        "select" => "combobox",
        "textarea" => "textbox",
        "th" => "columnheader",
        "td" => "cell",
        "tr" => "row",
        _ => return None,
    })
}

fn input_role(element: &Element) -> &'static str {
    let kind = element
        .attrs
        .get("type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into());
    match kind.as_str() {
        "button" | "submit" | "reset" => "button",
        "checkbox" => "checkbox",
        "radio" => "radio",
        "range" => "slider",
        _ => "textbox",
    }
}
