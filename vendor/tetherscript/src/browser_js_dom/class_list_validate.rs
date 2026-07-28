use super::*;

pub(super) fn required(args: &[JsValue], index: usize, method: &str) -> Result<String, String> {
    let value = args
        .get(index)
        .ok_or_else(|| format!("TypeError: classList.{method}: expected token"))?;
    token(value, method)
}

pub(super) fn token(value: &JsValue, method: &str) -> Result<String, String> {
    let token = value.display();
    check(&token, method)?;
    Ok(token)
}

pub(super) fn all(args: &[JsValue], method: &str) -> Result<Vec<String>, String> {
    args.iter().map(|value| token(value, method)).collect()
}

fn check(token: &str, method: &str) -> Result<(), String> {
    if token.is_empty() {
        return Err(format!("SyntaxError: classList.{method} token is empty"));
    }
    if token.chars().any(|ch| ch.is_ascii_whitespace()) {
        return Err(format!(
            "InvalidCharacterError: classList.{method} token contains whitespace"
        ));
    }
    Ok(())
}
