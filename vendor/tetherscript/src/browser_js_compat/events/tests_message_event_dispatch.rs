use super::super::*;

#[test]
fn message_event_family_dispatch_preserves_fields() {
    let result = eval_with_dom(
        "<div id='sink'></div>",
        "let el=document.getElementById('sink');let seen='';\
         el.addEventListener('message',function(e){seen=e.type+':'+e.target.id+\
         ':'+e.data.text+':'+e.origin+':'+e.lastEventId+':'+e.source.id+':'+\
         e.ports[0].id;});el.addEventListener('error',function(e){seen=seen+\
         '|'+e.message+':'+e.filename+':'+e.lineno+':'+e.colno+':'+e.error.kind;});\
         el.addEventListener('close',function(e){seen=seen+'|'+e.wasClean+':'+\
         e.code+':'+e.reason;});let ok1=el.dispatchEvent(MessageEvent('message',\
         {data:{text:'payload'},origin:'o',lastEventId:'7',source:{id:'s'},\
         ports:[{id:'p'}]}));let ok2=el.dispatchEvent(ErrorEvent('error',\
         {message:'fail',filename:'f.js',lineno:1,colno:2,error:{kind:'err'}}));\
         let ok3=el.dispatchEvent(CloseEvent('close',{wasClean:true,code:1001,\
         reason:'bye'}));seen+'|'+ok1+':'+ok2+':'+ok3;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "message:sink:payload:o:7:s:p|fail:f.js:1:2:err|true:1001:bye|true:true:true"
    );
}
