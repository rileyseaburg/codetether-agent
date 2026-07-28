use tetherscript::js;

#[test]
fn intrinsic_path_regex_drives_callback_replace() {
    let source = r#"let seen=[];
        let r=/[^%.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|%$))/g;
        let replace=Function.prototype.bind.call(Function.prototype.call,String.prototype.replace);
        let exec=Function.prototype.bind.call(Function.prototype.call,RegExp.prototype.exec);
        let tag=Object.prototype.toString.call(/x/);
        replace("%String.prototype.indexOf%",r,function(a,o,l,c){seen[seen.length]=o||a;});
        seen.join(".")+":"+(exec(/^%?[^%]*%?$/,"%String.prototype.indexOf%")!==null)+":"+tag;"#;

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("String.prototype.indexOf:true:[object RegExp]".into())
    );
}

#[test]
fn primitive_value_of_methods_support_call_bind() {
    let source = "let bind=function(fn){return fn.call.bind(fn);};\
        let n=bind(Number.prototype.valueOf)(4);\
        let s=bind(String.prototype.valueOf)('x');\
        let b=bind(Boolean.prototype.valueOf)(false);\
        let y=bind(Symbol.prototype.valueOf)(Symbol('x'));\
        let rejected=false; try{bind(Number.prototype.valueOf)({});}catch(e){rejected=true;}\
        n+':'+s+':'+b+':'+y+':'+rejected;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("4:x:false:Symbol(x):true".into())
    );
}

#[test]
fn object_has_own_property_static_supports_call() {
    let source = "let f=Object.hasOwnProperty; f.call({a:1},'a')+':'+Object.hasOwn({b:2},'b');";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("true:true".into())
    );
}
