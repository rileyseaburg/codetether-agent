use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn headers_delete_removes_normalized_names() {
    let value = eval(
        "let h=Headers([['X-A','1'],['X-B','2']]); \
         h.append('x-a','3'); h.delete('X-A'); \
         h.has('x-a') + '|' + h.get('x-b') + '|' + h.keys().join(',');",
    );
    assert_eq!(value, JsValue::String("false|2|x-b".into()));
}

#[test]
fn headers_keys_values_and_entries_are_deterministic_arrays() {
    let value = eval(
        "let h=Headers([['X-A','1'],['X-B','2']]); h.append('X-A','3'); \
         let rows=h.entries(); \
         h.keys().join(',')+'|'+h.values().join(',')+'|'+rows.length+'|' \
         +rows[0][0]+'='+rows[0][1]+'|'+rows[2][0]+'='+rows[2][1];",
    );
    assert_eq!(
        value,
        JsValue::String("x-a,x-b,x-a|1,2,3|3|x-a=1|x-a=3".into())
    );
}

#[test]
fn headers_for_each_uses_this_arg_and_passes_self() {
    let value = eval(
        "let h=Headers([['X-A','1'],['X-B','2']]); let ctx={tag:'ctx'}; \
         let seen=''; h.forEach(function(value,name,self){ \
         seen=seen+this.tag+':'+name+'='+value+':' + (self===h) + ';'; }, ctx); seen;",
    );
    assert_eq!(
        value,
        JsValue::String("ctx:x-a=1:true;ctx:x-b=2:true;".into())
    );
}
