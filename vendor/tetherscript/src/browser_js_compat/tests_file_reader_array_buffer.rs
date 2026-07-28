use super::super::*;

#[test]
fn file_reader_reads_array_buffer_bytes_and_events() {
    let result = eval_with_dom(
        "<main></main>",
        "let events=[]; let r=FileReader(); \
         r.onloadstart=function(e){ events.push(e.type+':'+r.readyState); }; \
         r.onload=function(e){ let a=r.result; events.push(e.type+':'+a.length+':'+a[0]+':'+a[3]); }; \
         r.onloadend=function(e){ events.push(e.type+':'+r.readyState); }; \
         r.readAsArrayBuffer(Blob(['hi ',Uint8Array([65,66])])); \
         events.join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("loadstart:1|load:5:104:65|loadend:2".into())
    );
}
