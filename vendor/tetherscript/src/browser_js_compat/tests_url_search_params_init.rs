use super::super::*;

#[test]
fn url_search_params_constructs_records_pairs_and_updates_size() {
    let result = eval_with_dom(
        "<main></main>",
        "let o=URLSearchParams({b:'2',a:'1'});\
         let p=URLSearchParams([['a','1'],['a','2'],['b','3']]);\
         let before=p.toString();let sizes=[p.size];p.append('c','4');sizes.push(p.size);\
         p.set('a','9');sizes.push(p.size);p.delete('b');sizes.push(p.size);\
         p.size=99;p.sort();sizes.push(p.size);\
         o.toString()+'|'+before+'|'+p.toString()+'|'+sizes.join(',')+'|'+p.toJSON();",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("a=1&b=2|a=1&a=2&b=3|a=9&c=4|3,4,3,2,2|a=9&c=4".into())
    );
}
