use super::super::*;

#[test]
fn response_constructor_fields_methods_and_clone_work() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Response('{\"ok\":true}',{status:201,statusText:'Created',headers:{'X-Test':'yes'}}); \
         let before=r.bodyUsed; let text=''; let json=''; \
         r.clone().text().then(function(v){ text=v; }); \
         r.json().then(function(v){ json=v.ok; }); \
         typeof Response+'|'+r.status+'|'+r.statusText+'|'+r.ok+'|'+r.url+'|'\
         +r.headers.get('x-test')+'|'+before+'>'+r.bodyUsed+'|'+text+'|'+json;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|201|Created|true||yes|false>true|{\"ok\":true}|true".into())
    );
}

#[test]
fn response_default_body_is_empty_string() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Response(); let out='x'; \
         r.text().then(function(v){ out=v; }); \
         r.status+':'+r.ok+':'+r.url+':'+out.length+':'+r.bodyUsed;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("200:true::0:true".into()));
}

#[test]
fn response_blob_returns_body_blob() {
    let result = eval_with_dom(
        "<main></main>",
        "let r=new Response('ABC',{headers:{'Content-Type':'text/custom'}});let c=r.clone();let out='';\
         c.blob().then(function(b){let t='';let a='';b.text().then(function(v){t=v;});b.arrayBuffer().then(function(v){a=v.length+':'+v[0]+':'+v[1]+':'+v[2];});out=b.size+':'+b.type+':'+t+':'+a;});\
         out+'|'+r.bodyUsed+'|'+c.bodyUsed;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("3:text/custom:ABC:3:65:66:67|false|true".into())
    );
}
