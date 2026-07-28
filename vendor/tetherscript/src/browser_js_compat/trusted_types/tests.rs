use super::super::*;

#[test]
fn trusted_types_global_exposes_policy_surface_and_probes() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=trustedTypes.createPolicy('agent',{}); \
         typeof trustedTypes+'|'+(trustedTypes===window.trustedTypes)+'|'\
         +p.name+'|'+typeof p.createHTML+'|'+typeof p.createScript+'|'\
         +typeof p.createScriptURL+'|'+trustedTypes.emptyHTML+'|'\
         +trustedTypes.emptyScript+'|'+trustedTypes.isHTML('x')+'|'\
         +trustedTypes.isScript('x')+'|'+trustedTypes.isScriptURL('x');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("object|true|agent|function|function|function|||false|false|false".into())
    );
}

#[test]
fn trusted_types_policy_calls_rules_and_falls_back_to_input() {
    let result = eval_with_dom(
        "<main></main>",
        "let ruled=trustedTypes.createPolicy('p',{ \
         createHTML:function(v){return '<b>'+v+'</b>';}, \
         createScript:function(){return 42;}, \
         createScriptURL:function(v){return {url:v};} }); \
         let plain=trustedTypes.createPolicy('plain'); \
         ruled.createHTML('x')+'|'+ruled.createScript('x')+'|'\
         +ruled.createScriptURL('/a.js')+'|'+plain.createHTML('<i>x</i>')+'|'\
         +plain.createScript(7)+'|'+plain.createScriptURL(null);",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("<b>x</b>|42|[object Object]|<i>x</i>|7|null".into())
    );
}
