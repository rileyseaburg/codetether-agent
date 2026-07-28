use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

fn eval(script: &str) -> Result<JsValue, String> {
    eval_with_dom("<div id='x' class='a b c'></div>", script).map(|result| result.value)
}

#[test]
fn class_list_validates_tokens() {
    let empty = eval("document.getElementById('x').classList.contains('');").unwrap_err();
    let space = eval("document.getElementById('x').classList.add('ok','bad token');").unwrap_err();
    let remove = eval("document.getElementById('x').classList.remove('bad token');").unwrap_err();
    let toggle = eval("document.getElementById('x').classList.toggle('bad token');").unwrap_err();
    let replace =
        eval("document.getElementById('x').classList.replace('a','bad token');").unwrap_err();
    assert!(empty.contains("SyntaxError: classList.contains token is empty"));
    assert!(space.contains("InvalidCharacterError: classList.add token contains whitespace"));
    assert!(remove.contains("InvalidCharacterError: classList.remove token contains whitespace"));
    assert!(toggle.contains("InvalidCharacterError: classList.toggle token contains whitespace"));
    assert!(replace.contains("InvalidCharacterError: classList.replace token contains whitespace"));
}
