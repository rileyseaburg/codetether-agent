use tetherscript::js;

#[test]
fn pako_shape_inflate_uses_native_zlib_fast_path() {
    let source = "let p={inflateRaw:1,ungzip:1,inflate:function(){return 'slow';}};\
        let b=[120,1,1,5,0,250,255,104,101,108,108,111,6,44,2,21];\
        p.inflate(b).map(function(x){return String.fromCharCode(x)}).join('');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("hello".into())
    );
}
