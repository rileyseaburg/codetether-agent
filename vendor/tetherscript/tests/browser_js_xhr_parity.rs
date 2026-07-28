use tetherscript::browser_js;
use tetherscript::js::JsValue;

#[test]
fn xhr_exposes_loadend_and_response_value() {
    let result = browser_js::eval_with_dom(
        "<main></main>",
        "let xhr=XMLHttpRequest();let events=[];let has='onloadend' in xhr;\
         xhr.onloadstart=function(){events.push('start:'+xhr.readyState);};\
         xhr.onreadystatechange=function(){if(xhr.readyState==4){events.push('rs:'+xhr.status);}};\
         xhr.onload=function(){events.push('load:'+xhr.response);};\
         xhr.onloadend=function(){console.log(has+':'+events.join('|'));};\
         xhr.open('get','/api/xhr');xhr.send();'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(
        result.console,
        vec!["true:start:1|rs:200|load:{\"url\":\"http://localhost/api/xhr\"}".to_string()]
    );
}

#[test]
fn xhr_json_response_type_parses_response() {
    let result = browser_js::eval_with_dom(
        "<main></main>",
        "let xhr=XMLHttpRequest();xhr.responseType='json';\
         xhr.onload=function(){console.log(xhr.response.url);};\
         xhr.open('get','/api/json');xhr.send();'sync';",
    )
    .unwrap();

    assert_eq!(
        result.console,
        vec!["http://localhost/api/json".to_string()]
    );
}
