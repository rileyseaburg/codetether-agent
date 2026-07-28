use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn document_command_methods_are_stable_noops() {
    let result = eval_with_dom(
        "<main id='root'></main>",
        "let root=document.getElementById('root');\
         document.execCommand('copy')+':'\
         +document.queryCommandSupported('copy')+':'\
         +document.queryCommandEnabled('copy')+':'\
         +root.ownerDocument.execCommand('copy');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("false:false:false:false".into())
    );
}
