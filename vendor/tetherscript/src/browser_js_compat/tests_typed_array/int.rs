use super::*;

#[test]
fn uint32_array_supports_realm_bundle_primitives() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new Uint32Array([4294967297,15]);let b=new Uint32Array([1,2]);\
         let c=new Uint32Array(new ArrayBuffer(8));\
         a.length+':'+a.BYTES_PER_ELEMENT+':'+Uint32Array.BYTES_PER_ELEMENT+':'+\
         a[0]+':'+a[1].toString(16).padStart(2,'0')+':'+b.join(',')+':'+\
         c.length+':'+ArrayBuffer.isView(a)+':'+(a instanceof Uint32Array)+':'+(a instanceof Uint8Array);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("2:4:4:1:0f:1,2:2:true:true:false".into())
    );
}

#[test]
fn float32_array_supports_geometry_bundle_primitives() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new Float32Array([1.5,true]);let b=new Float32Array(new ArrayBuffer(8));\
         a.length+':'+Float32Array.BYTES_PER_ELEMENT+':'+a[0]+':'+a[1]+':'+\
         b.length+':'+ArrayBuffer.isView(a)+':'+(a instanceof Float32Array)+':'+(a instanceof Uint32Array);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("2:4:1.5:1:2:true:true:false".into())
    );
}

#[test]
fn int32_array_supports_signed_bundle_primitives() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new Int32Array([2147483648,4294967295,7]);\
         a.length+':'+Int32Array.BYTES_PER_ELEMENT+':'+a.join(',')+':'+\
         (a instanceof Int32Array)+':'+(a instanceof Uint32Array);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("3:4:-2147483648,-1,7:true:false".into())
    );
}
