//! Pairing-code normalization compatible with the relay's ECMAScript rules.
use super::space;
use crate::Error;

pub(crate) fn normalize_code(code: &str) -> Result<String, Error> {
    let code = code
        .chars()
        .filter(|c| !space(*c) && *c != '-')
        .collect::<String>()
        .to_uppercase();
    if code.len() == 12 && code.bytes().all(|c| c.is_ascii_hexdigit()) {
        Ok(code)
    } else {
        Err(Error::Pairing)
    }
}
