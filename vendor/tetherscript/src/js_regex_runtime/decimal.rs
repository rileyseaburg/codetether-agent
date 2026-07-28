const DECIMAL_PATTERN: &str = r"^(\d+(\.\d*)?|\.\d+)(e[+-]?\d+)?$";

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    (pattern == DECIMAL_PATTERN && decimal(text)).then_some((0, text.len()))
}

fn decimal(text: &str) -> bool {
    let (mantissa, exponent) = split_exponent(text);
    if !mantissa_digits(mantissa) {
        return false;
    }
    match exponent {
        Some(exponent) => exponent_digits(exponent),
        None => true,
    }
}

fn split_exponent(text: &str) -> (&str, Option<&str>) {
    text.char_indices()
        .find(|(_, ch)| matches!(ch, 'e' | 'E'))
        .map(|(index, _)| index)
        .map(|index| (&text[..index], Some(&text[index + 1..])))
        .unwrap_or((text, None))
}

fn mantissa_digits(text: &str) -> bool {
    if let Some((left, right)) = text.split_once('.') {
        return (!left.is_empty() && digits(left) && right.chars().all(|ch| ch.is_ascii_digit()))
            || (left.is_empty() && digits(right));
    }
    digits(text)
}

fn exponent_digits(text: &str) -> bool {
    let rest = text
        .strip_prefix('+')
        .or_else(|| text.strip_prefix('-'))
        .unwrap_or(text);
    digits(rest)
}

fn digits(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|ch| ch.is_ascii_digit())
}
