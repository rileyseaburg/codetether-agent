use super::super::*;

#[test]
fn message_event_family_defaults() {
    let result = eval_with_dom(
        "",
        "let m=MessageEvent('message');let e=ErrorEvent('error');let c=CloseEvent('close');\
         typeof MessageEvent+':'+m.type+':'+(m.data===null)+':'+m.origin+','+\
         m.lastEventId+':'+(m.source===null)+':'+m.ports.length+':'+\
         e.message+','+e.filename+','+e.lineno+','+e.colno+','+(e.error===null)+\
         ':'+c.wasClean+','+c.code+','+c.reason+':'+typeof m.preventDefault+\
         ':'+m.defaultPrevented;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "function:message:true:,:true:0:,,0,0,true:false,0,:function:false"
    );
}
