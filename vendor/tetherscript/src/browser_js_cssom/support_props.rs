pub(super) fn supported(name: &str, value: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    let value = value.trim().to_ascii_lowercase();
    if name.is_empty() || value.is_empty() {
        return false;
    }
    match name.as_str() {
        "display" => ["block", "inline", "inline-block", "none", "flex"].contains(&value.as_str()),
        "position" => {
            ["static", "relative", "absolute", "fixed", "sticky"].contains(&value.as_str())
        }
        "color" | "background" | "background-color" | "border-color" => color(&value),
        "width" | "height" | "min-width" | "max-width" | "min-height" | "max-height" | "left"
        | "right" | "top" | "bottom" => value == "auto" || length(&value),
        "margin" | "margin-left" | "margin-right" | "margin-top" | "margin-bottom" => {
            box_lengths(&value, true)
        }
        "padding" | "padding-left" | "padding-right" | "padding-top" | "padding-bottom"
        | "border-width" => box_lengths(&value, false),
        "box-sizing" => matches!(value.as_str(), "content-box" | "border-box"),
        "overflow" => matches!(value.as_str(), "visible" | "hidden" | "scroll" | "auto"),
        "font-size" => length(&value) || matches!(value.as_str(), "small" | "medium" | "large"),
        "font-family" => true,
        "z-index" => value == "auto" || value.parse::<i64>().is_ok(),
        _ => false,
    }
}

fn box_lengths(value: &str, allow_auto: bool) -> bool {
    let parts = value.split_whitespace().collect::<Vec<_>>();
    (1..=4).contains(&parts.len())
        && parts
            .iter()
            .all(|part| length(part) || (allow_auto && *part == "auto"))
}

fn length(value: &str) -> bool {
    if value == "0" {
        return true;
    }
    ["px", "em", "rem", "%"].iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| number.parse::<f64>().is_ok())
    })
}

fn color(value: &str) -> bool {
    ["black", "white", "red", "green", "blue", "transparent"].contains(&value)
        || value.starts_with('#')
        || (value.starts_with("rgb(") && value.ends_with(')'))
}
