use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[path = "tests_props.rs"]
mod tests_props;
#[path = "tests_setters.rs"]
mod tests_setters;

fn eval(html: &str, script: &str) -> JsValue {
    eval_with_dom(html, script).unwrap().value
}
