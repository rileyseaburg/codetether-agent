use super::super::super::*;

#[test]
fn notification_close_and_listener_dispatch_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let n=new Notification('Hi');let log='';\
         n.onclose=function(e){log=log+'h:'+e.type+':'\
         +(this===n)+':'+(e.target===n)+';';};\
         n.addEventListener('close',function(e){log=log+'l:'\
         +this.closed+':'+e.currentTarget.title;});\
         let ret=n.close();log+'|'+ret+'|'+n.closed;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("h:close:true:true;l:true:Hi|true|true".into())
    );
}

#[test]
fn notification_dispatch_event_honors_remove_listener() {
    let result = eval_with_dom(
        "<main></main>",
        "let n=new Notification('Hi');let count=0;\
         function f(e){count=count+1;}n.addEventListener('show',f);\
         n.dispatchEvent({type:'show'});n.removeEventListener('show',f);\
         n.dispatchEvent({type:'show'});count;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::Number(1.0));
}
