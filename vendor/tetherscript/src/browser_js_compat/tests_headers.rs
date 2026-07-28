use super::super::*;

#[test]
fn headers_rows_delete_and_for_each_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let h=Headers([['X-A','1']]);\
         h.append('X-B','2');h.set('X-A','3');h.append('X-A','4');\
         let rows=h.entries();let keys=h.keys().join(',');\
         let values=h.values().join(',');let seen='';\
         h.forEach(function(v,k){seen=seen+k+'='+v+';';});\
         h.delete('x-b');h.has('X-B')+'|'+h.get('x-a')+'|'\
         +rows.length+'|'+rows[0][0]+'='+rows[0][1]+'|'\
         +keys+'|'+values+'|'+seen;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("false|3|3|x-b=2|x-b,x-a,x-a|2,3,4|x-b=2;x-a=3;x-a=4;".into())
    );
}
