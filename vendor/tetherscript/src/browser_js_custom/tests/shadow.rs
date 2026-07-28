use super::super::super::*;

#[test]
fn shadow_root_metadata_tracks_mode_host_and_focus_delegation() {
    let result = eval_with_dom(
        "<div id='h'></div><div id='c'></div>",
        "let h=document.getElementById('h'); let r=h.attachShadow({mode:'open',delegatesFocus:true});\
         let c=document.getElementById('c').attachShadow({mode:'closed'});\
         r.mode + ':' + r.host.id + ':' + r.delegatesFocus + ':' + h.shadowRoot.mode + ':' +\
         (document.getElementById('c').shadowRoot === null) + ':' + c.mode;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("open:h:true:open:true:closed".into())
    );
}
