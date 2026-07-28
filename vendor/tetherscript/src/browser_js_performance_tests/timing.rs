use super::super::*;

#[test]
fn performance_now_marks_and_measures_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let n0=performance.now(); let n1=performance.now();\
         performance.mark('start'); performance.mark('end');\
         performance.measure('work','start','end');\
         let e=performance.getEntriesByName('work','measure')[0];\
         ''+n0+':'+n1+':'+e.entryType+':'+e.duration+':'\
         +performance.getEntriesByType('mark').length;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("0:1:measure:1:2".into()));
}

#[test]
fn performance_clear_methods_remove_named_entries() {
    let result = eval_with_dom(
        "<main></main>",
        "performance.mark('a'); performance.mark('b');\
         performance.measure('m','a','b'); performance.clearMarks('a');\
         performance.clearMeasures(); performance.getEntriesByType('mark').length\
         + ':' + performance.getEntriesByType('measure').length;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("1:0".into()));
}
