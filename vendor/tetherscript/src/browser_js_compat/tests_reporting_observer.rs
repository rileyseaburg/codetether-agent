use super::super::super::super::*;

#[test]
fn reporting_observer_is_probeable_on_window_and_global() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new ReportingObserver();\
         [typeof ReportingObserver,ReportingObserver===window.ReportingObserver,\
         typeof a.observe,typeof a.disconnect,typeof a.takeRecords].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|true|function|function|function".into())
    );
}

#[test]
fn reporting_observer_observe_callback_gets_empty_records() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen='';let obs=new ReportingObserver(function(records,self){\
         seen=records.length+':' +(self===obs)+':' +(records!==obs.takeRecords());\
         },{types:['deprecation'],buffered:true});\
         let none=new ReportingObserver({types:['csp-violation']});none.observe();\
         let ret=obs.observe();let a=obs.takeRecords();let b=obs.takeRecords();\
         seen+'|'+ret+'|'+a.length+':'+b.length+':' +(a!==b)+'|'\
         +none.takeRecords().length+'|'+obs.disconnect();",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("0:true:true|undefined|0:0:true|0|undefined".into())
    );
}
