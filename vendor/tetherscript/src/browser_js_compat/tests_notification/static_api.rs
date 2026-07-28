use super::super::super::*;

#[test]
fn notification_static_permission_defaults_to_prompt() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let seen='';let p=Notification.requestPermission(function(s){seen='cb:'+s;});\
         p.then(function(s){seen=seen+':then:'+s;document.getElementById('out').textContent=seen;});\
         typeof Notification+'|'+(Notification===window.Notification)+'|'\
         +Notification.permission+'|'+seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|true|prompt|cb:prompt".into())
    );
    assert!(result.html.contains("cb:prompt:then:prompt"));
}
