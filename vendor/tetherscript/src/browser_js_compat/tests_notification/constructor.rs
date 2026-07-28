use super::super::super::*;

#[test]
fn notification_constructor_exposes_deterministic_fields() {
    let result = eval_with_dom(
        "<main></main>",
        "let data={id:7};\
         let n=new Notification('Hi',{body:'Body',tag:'t',data:data,\
         silent:true,renotify:true,requireInteraction:true});\
         [n.title,n.body,n.tag,n.data===data,n.silent,n.renotify,\
         n.requireInteraction,n.closed,n.onclick===null,n.onclose===null,\
         n.onerror===null,n.onshow===null].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("Hi|Body|t|true|true|true|true|false|true|true|true|true".into())
    );
}
