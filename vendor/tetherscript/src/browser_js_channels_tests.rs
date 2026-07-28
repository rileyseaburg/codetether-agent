use super::*;

#[test]
fn message_channel_queues_port_delivery() {
    let result = eval_with_dom(
        "<main></main>",
        "let ch=MessageChannel(); let log='';\
         ch.port2.onmessage=function(e){ log=log+'B'+e.data; ch.port2.postMessage('reply'); };\
         ch.port1.onmessage=function(e){ console.log(log+'A'+e.data); };\
         ch.port1.postMessage('ping'); log='S'; 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["SBpingAreply".to_string()]);
}

#[test]
fn broadcast_channel_delivers_to_same_origin_peers() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=BroadcastChannel('topic'); let b=BroadcastChannel('topic');\
         let c=BroadcastChannel('other');\
         b.onmessage=function(e){ console.log(e.origin+':'+e.data+':'+b.origin); };\
         c.onmessage=function(){ console.log('wrong'); }; a.postMessage('hi'); 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(
        result.console,
        vec!["http://localhost:hi:http://localhost".to_string()]
    );
}

#[test]
fn registered_worker_script_runs_without_threads() {
    let result = eval_with_dom(
        "<main></main>",
        "registerWorkerScript('/w.js', \"self.addEventListener('message', function(e){ self.postMessage('pong:'+e.data); });\");\
         let w=Worker('/w.js'); w.onmessage=function(e){ console.log(e.data); };\
         w.postMessage('ping'); 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["pong:ping".to_string()]);
}
