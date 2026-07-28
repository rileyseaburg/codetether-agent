use super::super::*;

fn run(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn text_decoder_fields_and_no_input_decode_are_deterministic() {
    let result = run(
        "let a=TextDecoder();let b=TextDecoder('utf8',{fatal:true,ignoreBOM:true});\
         a.encoding+':'+a.fatal+':'+a.ignoreBOM+'|'+b.encoding+':'\
         +b.fatal+':'+b.ignoreBOM+':'+b.decode();",
    );
    assert_eq!(
        result,
        JsValue::String("utf-8:false:false|utf-8:true:true:".into())
    );
}
