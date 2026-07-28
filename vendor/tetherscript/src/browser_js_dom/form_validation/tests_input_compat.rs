use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

fn eval(html: &str, script: &str) -> JsValue {
    eval_with_dom(html, script).unwrap().value
}

#[test]
fn input_default_and_indeterminate_properties_are_settable() {
    let html = "<input id='t' value='seed'><input id='c' type='checkbox' checked>";
    let script = "let t=document.getElementById('t');let c=document.getElementById('c');\
        let a=t.defaultValue+':'+t.value+':'+c.defaultChecked+':'\
        +c.checked+':'+c.indeterminate;t.defaultValue='changed';\
        c.defaultChecked=false;c.indeterminate=true;a+'|'+t.defaultValue\
        +':'+t.value+':'+t.getAttribute('value')+':'+c.defaultChecked\
        +':'+c.checked+':'+c.hasAttribute('checked')+':'+c.indeterminate;";
    assert_eq!(
        eval(html, script),
        JsValue::String(
            "seed:seed:true:true:false|changed:changed:changed:false:false:false:true".into()
        )
    );
}

#[test]
fn numeric_inputs_support_value_as_number_and_step_methods() {
    let html = "<input id='n' type='number' value='2' step='0.5'>\
        <input id='r' type='range' value='3' step='2'>\
        <input id='t' value='4'><input id='e' type='number'>";
    let script = "let n=document.getElementById('n');let r=document.getElementById('r');\
        let t=document.getElementById('t');let e=document.getElementById('e');\
        let a=n.valueAsNumber+':'+r.valueAsNumber+':'\
        +(t.valueAsNumber==t.valueAsNumber)+':'\
        +(e.valueAsNumber==e.valueAsNumber);n.stepUp();r.stepDown(2);\
        n.valueAsNumber=7.25;e.valueAsNumber=NaN;a+'|'+n.value+':'\
        +n.valueAsNumber+':'+r.value+':'+r.valueAsNumber+':'+e.value\
        +':'+(e.valueAsNumber==e.valueAsNumber);";
    assert_eq!(
        eval(html, script),
        JsValue::String("2:3:false:false|7.25:7.25:-1:-1::false".into())
    );
}
