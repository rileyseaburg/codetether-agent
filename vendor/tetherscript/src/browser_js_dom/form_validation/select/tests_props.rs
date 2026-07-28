use super::*;

#[test]
fn select_exposes_options_and_selected_state() {
    let html = "<select id='s'><optgroup><option id='a'>Alpha</option></optgroup>\
        <option id='b' name='bee' value='B' selected>Beta</option></select>";
    let script = "let s=document.getElementById('s');let o=s.options;\
        s.length+':'+o.length+':'+s.selectedIndex+':'+s.value+':'\
        +o[0].id+':'+o.item(1).text+':'+o.namedItem('bee').value+':'\
        +o[0].index+':'+o[1].selected;";
    assert_eq!(
        eval(html, script),
        JsValue::String("2:2:1:B:a:Beta:B:0:true".into())
    );
}
