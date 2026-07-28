use super::super::*;

#[test]
fn file_reader_dispatches_deterministic_text_and_data_url_events() {
    let result = eval_with_dom(
        "<main></main>",
        "let events=[]; let r=FileReader(); \
         r.onloadstart=function(e){ events.push(e.type+':'+r.readyState); }; \
         r.addEventListener('load',function(e){ events.push(e.type+':'+r.result); }); \
         r.onloadend=function(e){ events.push(e.type+':'+r.readyState); }; \
         r.readAsText(Blob(['abc'],{type:'text/plain'})); \
         let r2=FileReader(); r2.onload=function(){ events.push(r2.result); }; \
         r2.readAsDataURL(File(['A'],'a.txt',{type:'text/plain'})); \
         events.join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("loadstart:1|load:abc|loadend:2|data:text/plain;base64,QQ==".into())
    );
}
