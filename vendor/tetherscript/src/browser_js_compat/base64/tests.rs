use super::*;

fn eval(script: &str) -> Result<JsValue, String> {
    eval_with_dom("<main></main>", script).map(|result| result.value)
}

#[test]
fn btoa_encodes_binary_string() {
    assert_eq!(
        eval("btoa('hello');").unwrap(),
        JsValue::String("aGVsbG8=".into())
    );
}

#[test]
fn atob_decodes_whitespace_and_padding() {
    assert_eq!(
        eval("atob(' YW Jj\\nZA== ');").unwrap(),
        JsValue::String("abcd".into())
    );
}

#[test]
fn atob_btoa_round_trips_binary_byte_255() {
    assert_eq!(
        eval("let x=atob('/w=='); btoa(x)+':'+x.length;").unwrap(),
        JsValue::String("/w==:1".into())
    );
}

#[test]
fn invalid_inputs_are_rejected() {
    let bad_char = char::from_u32(0x100).unwrap();
    let btoa_err = eval(&format!("btoa('{bad_char}');")).unwrap_err();
    let atob_err = eval("atob('abc$');").unwrap_err();

    assert!(btoa_err.contains("btoa:"));
    assert!(atob_err.contains("atob: invalid base64 character"));
}
