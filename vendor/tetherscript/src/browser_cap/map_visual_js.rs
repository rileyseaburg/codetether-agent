//! JavaScript snippets for visual browser queries.

use super::super::call::BrowserCall;

pub(crate) fn eval(expression: String, scope: &'static str) -> BrowserCall {
    BrowserCall::new(
        "eval",
        scope,
        super::super::value::map_value(vec![
            ("action", super::super::value::str_value("eval")),
            ("expression", super::super::value::str_value(expression)),
        ]),
    )
}

pub(crate) fn visible(selector: &str) -> String {
    format!("(()=>{{let e=document.querySelector({});if(!e)return false;let r=e.getBoundingClientRect();let s=getComputedStyle(e);return !!(r.width&&r.height&&s.visibility!='hidden'&&s.display!='none')}})()", js_string(selector))
}

pub(crate) fn enabled(selector: &str) -> String {
    format!("(()=>{{let e=document.querySelector({});return !!(e&&!e.disabled&&e.getAttribute('aria-disabled')!='true')}})()", js_string(selector))
}

pub(crate) fn bounding_box(selector: &str) -> String {
    format!("(()=>{{let e=document.querySelector({});if(!e)return null;let r=e.getBoundingClientRect();return {{x:r.x,y:r.y,width:r.width,height:r.height}}}})()", js_string(selector))
}

fn js_string(text: &str) -> String {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}
