use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn attributes_indexed_and_named_lookup() {
    let result = eval_with_dom(
        "<div id='x' class='c' data-z='9'></div>",
        "let a=document.getElementById('x').attributes;\
         a.length+':'+a[0].name+'='+a[0].value\
         +':'+a.item(2).nodeName+'='+a.item(2).nodeValue\
         +':'+a.getNamedItem('class').value+':'+a.getNamedItem('missing');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("3:class=c:id=x:c:null".into())
    );
}

#[test]
fn attributes_for_each_uses_deterministic_order() {
    let result = eval_with_dom(
        "<div id='x' class='c' data-z='9'></div>",
        "let a=document.getElementById('x').attributes;let seen='';\
         a.forEach(function(attr,i,map){seen=seen+i+attr.name+(map===a);});seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("0classtrue1data-ztrue2idtrue".into())
    );
}
