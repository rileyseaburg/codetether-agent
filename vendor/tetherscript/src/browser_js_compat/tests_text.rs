use super::super::*;

fn run(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn text_encoder_decoder_round_trip_bytes() {
    let result = run(
        "let enc=TextEncoder();let bytes=enc.encode('Az');let text=TextDecoder().decode(bytes);\
         bytes.length+':'+bytes[0]+':'+bytes[1]+':'+text+':'+enc.encoding;",
    );
    assert_eq!(result, JsValue::String("2:65:122:Az:utf-8".into()));
}

#[test]
fn text_encoder_encode_into_writes_full_utf8_bytes() {
    let result = run(
        "let s=TextDecoder().decode([194,162]);let d=Uint8Array(5);\
         let r=TextEncoder().encodeInto('A'+s,d);\
         let o={length:2};let q=TextEncoder().encodeInto('hi',o);\
         r.read+':'+r.written+':'+d.join('-')+'|'+q.read+':'\
         +q.written+':'+o[0]+'-'+o[1];",
    );
    assert_eq!(
        result,
        JsValue::String("2:3:65-194-162-0-0|2:2:104-105".into())
    );
}

#[test]
fn text_encoder_encode_into_stops_before_partial_scalar() {
    let result = run(
        "let s=TextDecoder().decode([194,162]);let d=Uint8Array(2);\
         let r=TextEncoder().encodeInto('A'+s+'B',d);\
         r.read+':'+r.written+':'+d.join('-');",
    );
    assert_eq!(result, JsValue::String("1:1:65-0".into()));
}

#[test]
fn text_encoder_default_input_is_empty() {
    let result = run(
        "let a=TextEncoder().encode();let d=Uint8Array(3);\
         let r=TextEncoder().encodeInto(undefined,d);\
         a.length+':'+r.read+':'+r.written+':'+d.join('-');",
    );
    assert_eq!(result, JsValue::String("0:0:0:0-0-0".into()));
}
