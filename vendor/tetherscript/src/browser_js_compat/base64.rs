use super::*;

#[path = "base64/alphabet.rs"]
mod alphabet;
#[path = "base64/clean.rs"]
mod clean;
#[path = "base64/decode.rs"]
mod decode;
#[path = "base64/encode.rs"]
mod encode;

#[cfg(test)]
#[path = "base64/tests.rs"]
mod tests;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert("btoa".into(), native("btoa", Some(1), btoa));
    window.insert("atob".into(), native("atob", Some(1), atob));
}

fn btoa(args: &[JsValue]) -> Result<JsValue, String> {
    let input = args.first().unwrap_or(&JsValue::Undefined).display();
    Ok(JsValue::String(encode::text(&input)?))
}

fn atob(args: &[JsValue]) -> Result<JsValue, String> {
    let input = args.first().unwrap_or(&JsValue::Undefined).display();
    decode::text(&input).map(JsValue::String)
}
