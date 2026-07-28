use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn navigator_permissions_query_resolves_prompt_status() {
    let value = eval(
        "let seen='';let p=navigator.permissions.query({name:'geolocation'});\
         let next=p.then(function(s){seen=[s.state,s.name,''+s.onchange,\
         typeof s.addEventListener,typeof s.removeEventListener,\
         typeof s.dispatchEvent,''+s.addEventListener('change',function(){}),\
         ''+s.removeEventListener('change',function(){}),\
         s.dispatchEvent({type:'change'})].join('|');return s.state+':'+s.name;});\
         [p.__promise_state,next.__promise_state,next.__promise_value,seen].join(';');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "fulfilled;fulfilled;prompt:geolocation;\
             prompt|geolocation|null|function|function|function|undefined|undefined|true"
                .into()
        )
    );
}

#[test]
fn navigator_permissions_query_defaults_missing_name() {
    let value = eval(
        "let seen='unset';navigator.permissions.query({}).then(function(s){\
         seen=s.name+':'+s.state;});seen;",
    );

    assert_eq!(value, JsValue::String(":prompt".into()));
}
