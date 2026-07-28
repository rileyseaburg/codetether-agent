use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn matches_supports_compound_attrs_and_aliases() {
    let result = eval_with_dom(
        "<main><button id='save' class='primary' data-role='action'></button><button id='cancel'></button></main>",
        "let s=document.getElementById('save');let c=document.getElementById('cancel');\
         s.matches(\"button.primary[data-role='action']\")+':'\
         +c.matches(\"button.primary\")+':'\
         +s.webkitMatchesSelector(\"button[data-role='action']\")+':'\
         +s.msMatchesSelector(\"#save.primary\");",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("true:false:true:true".into()));
}

#[test]
fn closest_returns_self_ancestor_or_null() {
    let result = eval_with_dom(
        "<main id='app' data-shell='x'><section><button id='go' data-role='action'></button></section></main>",
        "let g=document.getElementById('go');let self=g.closest(\"button[data-role='action']\");\
         let ancestor=g.closest(\"main[data-shell='x']\");let none=g.closest('form');\
         self.id+':'+ancestor.id+':'+(none===null);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("go:app:true".into()));
}

#[test]
fn detached_closest_uses_detached_subtree() {
    let result = eval_with_dom(
        "<main></main>",
        "let host=document.createElement('div');host.setAttribute('data-host','x');\
         let child=host.appendChild(document.createElement('span'));\
         child.closest(\"div[data-host='x']\").getAttribute('data-host')+':'\
         +child.matches('span');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("x:true".into()));
}
