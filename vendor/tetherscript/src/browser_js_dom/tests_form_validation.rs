use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn required_controls_update_validity_and_form_result() {
    let html = "<form id='f'><input id='i' required><textarea id='t' required></textarea><select id='s' required><option value=''>Pick</option><option value='x'>X</option></select></form>";
    let script = "let f=document.getElementById('f'); let i=document.getElementById('i'); let t=document.getElementById('t'); let s=document.getElementById('s'); let before=f.checkValidity()+':'+i.validationMessage+':'+s.validity.valueMissing; i.value='a'; t.value='b'; s.value='x'; before+'|'+f.reportValidity()+':'+i.validationMessage;";
    let result = eval_with_dom(html, script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String("false:Please fill out this field.:true|true:".into())
    );
}

#[test]
fn custom_validity_refreshes_message_and_validity_state() {
    let script = "let i=document.getElementById('i'); i.setCustomValidity('Bad value'); let a=i.checkValidity()+':'+i.validity.customError+':'+i.validationMessage; i.setCustomValidity(''); a+'|'+i.checkValidity()+':'+i.validationMessage;";
    let result = eval_with_dom("<input id='i'>", script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String("false:true:Bad value|true:".into())
    );
}

#[test]
fn email_and_url_inputs_report_type_mismatch() {
    let html = "<input id='e' type='email' value='nope'><input id='u' type='url' value='notaurl'>";
    let script = "let e=document.getElementById('e'); let u=document.getElementById('u'); e.checkValidity()+':'+e.validity.typeMismatch+':'+e.validationMessage+'|'+u.checkValidity()+':'+u.validity.typeMismatch+':'+u.validationMessage;";
    let result = eval_with_dom(html, script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "false:true:Please enter a valid email address.|false:true:Please enter a valid URL."
                .into()
        )
    );
}
