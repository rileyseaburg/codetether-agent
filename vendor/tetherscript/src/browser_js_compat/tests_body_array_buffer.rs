use super::super::*;

#[test]
fn request_array_buffer_marks_body_used_and_returns_bytes() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Request('/api',{body:'Az'});let out='';\
         r.arrayBuffer().then(function(v){out=v.length+':'+v[0]+':'+v[1];});\
         out+'|'+r.bodyUsed;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("2:65:122|true".into()));
}

#[test]
fn response_array_buffer_clone_has_independent_body_used() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Response('Hi');let c=r.clone();let out='';\
         c.arrayBuffer().then(function(v){out=v.length+':'+v[0]+':'+v[1];});\
         out+'|'+r.bodyUsed+'|'+c.bodyUsed;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("2:72:105|false|true".into()));
}
