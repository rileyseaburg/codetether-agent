use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[path = "tests_class_list_edges.rs"]
mod tests_class_list_edges;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<div id='x' class='a b c'></div>", script)
        .unwrap()
        .value
}

#[test]
fn class_list_numeric_indexes_refresh_after_mutations() {
    let value = eval(
        "let c=document.getElementById('x').classList;\
         let before=c[0]+','+c[1]+','+c[2];\
         c.remove('b','c');let shrink=c.length+':'+c[0]+':' +(c[1]===undefined);\
         c.add('d');let added=c[1];c.toggle('a',false);\
         let toggled=c[0]+':' +(c[1]===undefined);c.value='x y';\
         before+'|'+shrink+'|'+added+'|'+toggled+'|'+c[0]+':'+c[1]+':' +(c[2]===undefined);",
    );
    assert_eq!(
        value,
        JsValue::String("a,b,c|1:a:true|d|d:true|x:y:true".into())
    );
}

#[test]
fn class_list_for_each_passes_args_and_this_arg() {
    let value = eval(
        "let c=document.getElementById('x').classList;let ctx={tag:'T'};let seen='';\
         c.forEach(function(token,index,self){\
         seen=seen+this.tag+':'+index+':'+token+':' +(self===c)+';';},ctx);seen;",
    );
    assert_eq!(
        value,
        JsValue::String("T:0:a:true;T:1:b:true;T:2:c:true;".into())
    );
}

#[test]
fn class_list_keys_values_and_entries_return_arrays() {
    let value = eval(
        "let c=document.getElementById('x').classList;let rows=c.entries();\
         c.keys().join(',')+'|'+c.values().join(',')+'|'+rows.length+'|'\
         +rows[0][0]+':'+rows[0][1]+'|'+rows[2][0]+':'+rows[2][1];",
    );
    assert_eq!(value, JsValue::String("0,1,2|a,b,c|3|0:a|2:c".into()));
}
