use super::*;

#[test]
fn select_selected_index_and_value_setters_update_attrs() {
    let html = "<select id='s'><option value='A'>Alpha</option>\
        <option id='b' value='B'>Beta</option><option value='C'>Gamma</option></select>";
    let script = "let s=document.getElementById('s');s.selectedIndex=2;\
        let a=s.value+':'+s.selectedIndex+':'+s.options[2].selected+':'\
        +s.options[0].selected;s.selectedIndex=-1;let b=s.value+':'\
        +s.selectedIndex+':'+s.options[2].selected;s.value='B';let c=s.value\
        +':'+s.selectedIndex+':'+s.options[1].selected;s.value='missing';\
        a+'|'+b+'|'+c+'|'+s.value+':'+s.selectedIndex;";
    assert_eq!(
        eval(html, script),
        JsValue::String("C:2:true:false|:-1:false|B:1:true|B:1".into())
    );
}

#[test]
fn option_selected_setter_and_required_form_data_interact() {
    let html = "<form id='f'><select id='s' name='choice' required>\
        <option value=''>Pick</option><option id='b' value='B'>Beta</option></select></form>";
    let script = "let f=document.getElementById('f');let s=document.getElementById('s');\
        let before=f.checkValidity()+':'+s.validationMessage;\
        s.options[1].selected=true;before+'|'+f.checkValidity()+':'\
        +s.value+':'+s.selectedIndex+':'+s.options[1].selected+':'\
        +new FormData(f).get('choice');";
    assert_eq!(
        eval(html, script),
        JsValue::String("false:Please fill out this field.|true:B:1:true:B".into())
    );
}
