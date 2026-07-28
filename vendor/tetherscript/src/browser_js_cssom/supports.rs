use super::support_props;

pub(super) fn args(args: &[crate::js::JsValue]) -> bool {
    match args {
        [property, value, ..] => support_props::supported(&property.display(), &value.display()),
        [condition] => condition_text(&condition.display()),
        _ => false,
    }
}

pub(super) fn condition_text(source: &str) -> bool {
    eval(source.trim())
}

fn eval(source: &str) -> bool {
    let text = source.trim().to_ascii_lowercase();
    if text.is_empty() {
        return false;
    }
    if let Some(rest) = text.strip_prefix("not ") {
        return !eval(rest);
    }
    if text.contains(" or ") {
        return text.split(" or ").any(eval);
    }
    if text.contains(" and ") {
        return text.split(" and ").all(eval);
    }
    let text = unwrap_parens(&text);
    let Some((name, value)) = text.split_once(':') else {
        return false;
    };
    support_props::supported(name, value)
}

fn unwrap_parens(mut text: &str) -> &str {
    loop {
        let trimmed = text.trim();
        if !(trimmed.starts_with('(') && trimmed.ends_with(')')) {
            return trimmed;
        }
        text = &trimmed[1..trimmed.len() - 1];
    }
}
