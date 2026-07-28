use super::super::*;

#[test]
fn request_body_methods_mark_only_read_object() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Request('/api/post',{method:'post',headers:{'Content-Type':'application/json'},body:'{\"ok\":true}'});\
         let c=r.clone();let text='';let ok='';\
         r.text().then(function(v){text=v;});let after=r.bodyUsed+':'+c.bodyUsed;\
         c.json().then(function(v){ok=v.ok;});\
         r.url+'|'+r.method+'|'+r.headers.get('content-type')+'|'+r.body+'|'\
         +after+'|'+c.bodyUsed+'|'+text+'|'+ok;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "/api/post|POST|application/json|{\"ok\":true}|true:false|true|{\"ok\":true}|true"
                .into()
        )
    );
}

#[test]
fn request_clone_uses_current_headers_and_unused_body() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Request('/api/a',{headers:{'X-Old':'1'},body:'payload'});\
         r.headers.set('X-Old','2');r.headers.set('X-New','3');let c=r.clone();\
         c.headers.get('x-old')+'|'+c.headers.get('x-new')+'|'+c.bodyUsed+'|'+c.body;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("2|3|false|payload".into()));
}

#[test]
fn request_blob_returns_body_blob_and_leaves_clone_unused() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Request('/api',{headers:{'Content-Type':'text/plain'},body:'Hi'});let c=r.clone();let out='';\
         r.blob().then(function(b){let t='';let a='';b.text().then(function(v){t=v;});b.arrayBuffer().then(function(v){a=v.length+':'+v[0]+':'+v[1];});out=b.size+':'+b.type+':'+t+':'+a;});\
         out+'|'+r.bodyUsed+'|'+c.bodyUsed;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("2:text/plain:Hi:2:72:105|true|false".into())
    );
}
