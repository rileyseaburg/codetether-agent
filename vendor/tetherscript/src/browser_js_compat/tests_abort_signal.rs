use super::*;

#[test]
fn abort_signal_notifies_once_and_preserves_first_reason() {
    let result = eval_with_dom(
        "<main></main>",
        "let ac=AbortController(); let s=ac.signal; let seen='';\
         function keep(e){ seen=seen+'L'+e.type+':' +(e.target===s)+':' +(this===s)+':' +s.reason+';'; }\
         function gone(){ seen=seen+'G'; }\
         s.addEventListener('abort', keep); s.addEventListener('abort', gone);\
         s.removeEventListener('abort', gone);\
         s.onabort=function(e){ seen=seen+'H'+e.type+':' +(e.currentTarget===s)+':' +s.reason+';'; };\
         ac.abort('first'); ac.abort('second'); seen + '|' + s.aborted + ':' + s.reason;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "Labort:true:true:first;Habort:true:first;|true:first"
    );
}

#[test]
fn abort_signal_dispatch_event_accepts_abort_events() {
    let result = eval_with_dom(
        "<main></main>",
        "let s=AbortController().signal; let seen='';\
         s.addEventListener('abort', function(e){ seen=e.type+':' +s.aborted+':' +e.cancelable; e.preventDefault(); });\
         let ok=s.dispatchEvent(Event('abort',{cancelable:true}));\
         seen + ':' + ok + ':' + s.aborted;",
    )
    .unwrap();
    assert_eq!(result.value.display(), "abort:false:true:false:false");
}
