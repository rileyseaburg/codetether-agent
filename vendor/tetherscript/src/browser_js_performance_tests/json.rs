use super::super::*;

#[test]
fn performance_entry_to_json_snapshots_mark_and_measure() {
    let result = eval_with_dom(
        "<main></main>",
        "performance.mark('start'); performance.mark('end');\
         performance.measure('work','start','end');\
         let a=performance.getEntriesByName('start','mark')[0].toJSON();\
         let b=performance.getEntriesByName('work','measure')[0].toJSON();\
         [a.name,a.entryType,a.startTime,a.duration,\
          b.name,b.entryType,b.startTime,b.duration].join(':');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("start:mark:0:0:work:measure:0:1".into())
    );
}

#[test]
fn performance_observer_entry_to_json_snapshots_record() {
    let result = eval_with_dom(
        "<main></main>",
        "let obs=PerformanceObserver(function(list){\
           let j=list.getEntries()[0].toJSON();\
           console.log([j.name,j.entryType,j.startTime,j.duration].join(':'));\
         }); obs.observe({entryTypes:['mark']}); performance.mark('ready'); 'ok';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("ok".into()));
    assert_eq!(result.console, vec!["ready:mark:0:0".to_string()]);
}

#[test]
fn performance_to_json_exposes_stable_time_origin() {
    let result = eval_with_dom(
        "<main></main>",
        "let before=performance.now(); let j=performance.toJSON();\
         let after=performance.now(); [j.timeOrigin,j.now===undefined,before,after].join(':');",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("0:true:0:1".into()));
}
