use super::super::*;

#[test]
fn response_json_static_sets_body_headers_and_init() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=Response.json({ok:true,count:2});let at='';a.text().then(function(v){at=v;});\
         let b=Response.json('hi',{status:201,statusText:'Made',headers:{'Content-Type':'text/custom'}});\
         let bt='';b.text().then(function(v){bt=v;});\
         a.status+'|'+a.headers.get('content-type')+'|'+at+'|'+a.bodyUsed+'>'\
         +b.status+'|'+b.statusText+'|'+b.headers.get('content-type')+'|'+bt;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "200|application/json|{\"count\":2,\"ok\":true}|true>201|Made|text/custom|\"hi\""
                .into()
        )
    );
}

#[test]
fn response_redirect_and_error_static_helpers_work() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=Response.redirect('/next',301);let fallback=Response.redirect('/bad',200);\
         let e=Response.error();let body='x';e.text().then(function(v){body=v;});\
         r.status+'|'+r.ok+'|'+r.headers.get('location')+'|'+fallback.status+'>'\
         +e.type+'|'+e.status+'|'+e.ok+'|'+body.length+'|'+e.bodyUsed;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("301|false|/next|302>error|0|false|0|true".into())
    );
}
