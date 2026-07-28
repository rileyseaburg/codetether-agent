use super::super::*;

#[test]
fn uint16_array_supports_realm_bundle_primitives() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new Uint16Array([65537,-1,true]);\
         let b=new Uint16Array(new ArrayBuffer(8));\
         a.length+':'+Uint16Array.BYTES_PER_ELEMENT+':'+a.join(',')+':'+\
         b.length+':'+ArrayBuffer.isView(a)+':'+(a instanceof Uint16Array)+':'+\
         (a instanceof Uint32Array);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("3:2:1,65535,1:4:true:true:false".into())
    );
}
