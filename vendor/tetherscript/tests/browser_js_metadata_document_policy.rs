use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn document_policy_probe_objects_are_available() {
    let value = eval(
        "let f=document.featurePolicy;let p=document.permissionsPolicy;\
         [typeof f,typeof p,typeof f.allowedFeatures,typeof p.features,\
         f.allowedFeatures().join(','),p.features().join(','),\
         f.allowsFeature('geolocation'),p.allowsFeature('camera','https://x.test'),\
         f.getAllowlistForFeature('microphone').join(','),\
         p.getAllowlistForFeature('payment').length].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String("object|object|function|function|||false|false||0".into())
    );
}

#[test]
fn document_policy_probe_arrays_are_fresh_and_empty() {
    let value = eval(
        "let a=document.featurePolicy.features();\
         a.push('geolocation');\
         a.length+'|'+document.featurePolicy.features().length+'|'\
         +document.permissionsPolicy.allowedFeatures().length;",
    );

    assert_eq!(value, JsValue::String("1|0|0".into()));
}
