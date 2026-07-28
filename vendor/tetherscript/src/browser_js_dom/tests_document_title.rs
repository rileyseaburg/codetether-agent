use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_title_reads_and_updates_existing_title() {
    let result = eval_with_dom(
        "<head><title>Old</title></head><main></main>",
        "let before=document.title;document.title='New & Better';\
         before+'|'+document.title+'|'+document.querySelector('title').textContent;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("Old|New & Better|New & Better".into())
    );
    assert!(result.html.contains("<title>New &amp; Better</title>"));
}

#[test]
fn document_title_creates_missing_head_and_title() {
    let result = eval_with_dom(
        "<main id='app'></main>",
        "let before=document.title;document.title='Made';\
         before+'|'+document.title+'|'+document.querySelector('head').firstChild.nodeName\
         +'|'+document.querySelector('title').textContent;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("|Made|title|Made".into()));
    assert!(result.html.starts_with("<head><title>Made</title></head>"));
}
