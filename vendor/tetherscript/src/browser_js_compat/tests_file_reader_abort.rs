use super::super::*;

#[test]
fn file_reader_abort_updates_state_and_dispatches_once() {
    let result = eval_with_dom(
        "<main></main>",
        "let events=[]; let idle=FileReader(); \
         idle.onabort=function(){ events.push('idle-abort'); }; idle.abort(); \
         events.push('idle:'+idle.readyState+':'+idle.result+':'+idle.error); \
         let r=FileReader(); \
         r.onloadstart=function(e){ events.push(e.type+':'+r.readyState+':'+r.result); r.abort(); events.push('after:'+r.readyState+':'+r.result+':'+r.error); }; \
         r.onabort=function(e){ events.push(e.type+':'+r.readyState+':'+r.result+':'+r.error); }; \
         r.onload=function(){ events.push('load'); }; \
         r.onloadend=function(e){ events.push(e.type+':'+r.readyState+':'+r.result+':'+r.error); }; \
         r.readAsText(Blob(['abc'])); r.abort(); \
         events.push('done:'+r.readyState+':'+r.result+':'+r.error); events.join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "idle:0:null:null|loadstart:1:null|abort:2:null:AbortError|\
             loadend:2:null:AbortError|after:2:null:AbortError|\
             done:2:null:AbortError"
                .into()
        )
    );
}
