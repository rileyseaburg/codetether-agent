use super::*;

#[test]
fn navigator_exposes_detection_capability_metadata() {
    let result = eval_with_dom(
        "<main></main>",
        "navigator.hardwareConcurrency + ':' + navigator.deviceMemory + ':'\
         + navigator.vendor + ':' + navigator.product + ':'\
         + navigator.maxTouchPoints + ':' + navigator.webdriver;",
    )
    .unwrap();

    assert_eq!(result.value.display(), "4:8:TetherScript:Gecko:0:false");
}

#[test]
fn navigator_network_information_dispatches_change_events() {
    let result = eval_with_dom(
        "<main></main>",
        "let c=navigator.connection; let seen='';\
         function gone(){ seen=seen+'gone'; }\
         c.addEventListener('change', function(e){ seen=seen+'L'+e.type+':'\
         +(e.target===c)+':' +(this===c)+';'; });\
         c.addEventListener('change', gone); c.removeEventListener('change', gone);\
         c.onchange=function(e){ seen=seen+'H'+(e.currentTarget===c)+';'; };\
         let ok=c.dispatchEvent(Event('change'));\
         (c===navigator.networkInformation)+ '|' + c.effectiveType + ':'\
         + c.downlink + ':' + c.rtt + ':' + c.saveData + '|' + seen + ok;",
    )
    .unwrap();

    assert_eq!(result.value.display(), "true|4g:10:50:false|Lchange:true:true;Htrue;true");
}
