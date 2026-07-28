use super::*;

#[test]
fn uint8_array_supports_buffer_feature_probe_shape() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=Uint8Array; let be=new a(1); let ge={foo:function(){return 42;}};\
         Object.setPrototypeOf(ge,a.prototype); Object.setPrototypeOf(be,ge);\
         (be.foo()===42)+':'+(be instanceof Uint8Array)+':'+ArrayBuffer.isView(be);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("true:true:true".into()));
}
