use super::super::*;

#[test]
fn message_event_family_init_values() {
    let result = eval_with_dom(
        "",
        "let src={id:'src'};let port={id:'p'};let err={kind:'boom'};\
         let m=MessageEvent('message',{data:{text:'hi'},origin:'https://a',\
         lastEventId:'42',source:src,ports:[port],bubbles:true});\
         let e=ErrorEvent('error',{message:'bad',filename:'app.js',lineno:9,\
         colno:4,error:err});let c=CloseEvent('close',{wasClean:true,\
         code:1000,reason:'done'});m.data.text+':'+m.origin+':'+\
         m.lastEventId+':'+m.source.id+':'+m.ports[0].id+':'+m.bubbles+\
         '|'+e.message+':'+e.filename+':'+e.lineno+':'+e.colno+':'+\
         e.error.kind+'|'+c.wasClean+':'+c.code+':'+c.reason;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "hi:https://a:42:src:p:true|bad:app.js:9:4:boom|true:1000:done"
    );
}
