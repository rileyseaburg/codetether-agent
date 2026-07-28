pub(super) fn mismatch(input_type: &str, value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    match input_type {
        "email" => !email(value),
        "url" => !url(value),
        _ => false,
    }
}

fn email(value: &str) -> bool {
    if value.chars().any(char::is_whitespace) {
        return false;
    }
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn url(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once(':') else {
        return false;
    };
    !scheme.is_empty()
        && scheme.chars().all(|ch| ch.is_ascii_alphabetic())
        && !rest.trim_start_matches('/').is_empty()
}
