use tetherscript::js;

#[test]
fn array_prototype_map_call_handles_array_like_receivers() {
    let source = "Array.prototype.map.call(['1','2'], Number).join(',');";

    assert_eq!(js::eval(source).unwrap(), js::JsValue::String("1,2".into()));
}

#[test]
fn array_find_index_supports_direct_and_prototype_calls() {
    let source = "let direct=['a','bb'].findIndex(function(x){return x.length===2;});\
        let called=Array.prototype.findIndex.call({0:'x',1:'yy',length:2}, x=>x.length===3);\
        direct+':'+called;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("1:-1".into())
    );
}

#[test]
fn array_reduce_right_supports_router_outlet_chains() {
    let source = "let direct=['a','b','c'].reduceRight((acc,x)=>acc+x,'');\
        let called=Array.prototype.reduceRight.call({0:'x',1:'y',length:2},\
        function(acc,x,i){return acc+x+i;},''); direct+':'+called;";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("cba:y1x0".into())
    );
}
