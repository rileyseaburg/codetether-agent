use crate::Error;
use codetether_companion_protocol::SessionInput;
mod code;
pub(crate) use code::normalize_code;

// ECMAScript whitespace matches the existing relay's trim() and /\s/ behavior.
fn space(c: char) -> bool {
    matches!(c, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' |
        '\u{205f}' | '\u{3000}' | '\u{feff}')
}
pub(crate) fn validate(mut input: SessionInput) -> Result<SessionInput, Error> {
    let valid_part = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"_.:-".contains(&c))
    };
    let valid_model = input
        .model
        .split_once('/')
        .is_some_and(|(provider, model)| {
            valid_part(provider)
                && !model.is_empty()
                && model
                    .split('/')
                    .all(|part| part.is_empty() || valid_part(part))
        });
    if !valid_model
        || input.model.len() > 200
        || input.prompt.encode_utf16().count() > 2000
        || input.prompt.trim_matches(space).is_empty()
        || !(15..=300).contains(&input.interval_seconds)
    {
        return Err(Error::Input);
    }
    input.prompt = input.prompt.trim_matches(space).into();
    Ok(input)
}
