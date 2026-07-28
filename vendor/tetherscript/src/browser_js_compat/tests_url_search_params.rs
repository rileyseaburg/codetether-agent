use super::super::*;

#[test]
fn url_search_params_collection_methods_preserve_order() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=URLSearchParams('b=2&a=1&a=3&empty=');\
         p.append('a','4');let all=p.getAll('a').join(',');let rows=p.entries();\
         let seen='';p.forEach(function(v,k,self){seen=seen+k+'='+v+':' +(self===p)+';';});\
         p.delete('empty');\
         all+'|'+p.keys().join(',')+'|'+p.values().join(',')+'|'\
         +rows[1][0]+'='+rows[1][1]+'|'+p.has('empty')+'|'+seen+'|'+p.toString();",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "1,3,4|b,a,a,a|2,1,3,4|a=1|false|\
             b=2:true;a=1:true;a=3:true;empty=:true;a=4:true;|b=2&a=1&a=3&a=4"
                .into()
        )
    );
}

#[test]
fn url_search_params_sort_is_stable_and_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=URLSearchParams('z=9&a=1&b=2&a=3');\
         p.sort();let rows=p.entries();\
         p.toString()+'|'+rows[0][0]+'='+rows[0][1]+'|'+p.getAll('a').join(',');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("a=1&a=3&b=2&z=9|a=1|1,3".into())
    );
}
