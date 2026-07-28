use super::*;

#[test]
fn uint8_array_views_array_buffers_and_mutates_bytes() {
    let result = eval_with_dom(
        "<main></main>",
        "let b=new ArrayBuffer(4); let v=new Uint8Array(b); v.set([7,8],1);\
         v.copyWithin(0,1,3); v.fill(9); v.length+':'+v.join(',');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("4:9,9,9,9".into()));
}

#[test]
fn array_buffer_allows_missing_length_and_exposes_brand() {
    let result = eval_with_dom(
        "<main></main>",
        "let b=new ArrayBuffer(); b.byteLength+':'+Object.prototype.toString.call(b);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("0:[object ArrayBuffer]".into())
    );
}
