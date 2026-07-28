use super::*;

#[test]
fn lifecycle_event_defaults_match_browser_shapes() {
    let result = eval_with_dom(
        "",
        "let p=PopStateEvent('pop');let h=HashChangeEvent('hashchange');\
         let t=PageTransitionEvent('pageshow');let b=BeforeUnloadEvent('beforeunload');\
         let r=ProgressEvent('progress');\
         (p.state===null)+':'+h.oldURL+','+h.newURL+':'+t.persisted+':' +\
         b.returnValue+':'+r.lengthComputable+','+r.loaded+','+r.total+':' +\
         typeof r.preventDefault+':'+r.defaultPrevented;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "true:,:false::false,0,0:function:false"
    );
}

#[test]
fn lifecycle_event_constructors_read_init_fields() {
    let result = eval_with_dom(
        "",
        "let state={id:7};let p=PopStateEvent('pop',{state:state,bubbles:true});\
         let h=HashChangeEvent('hashchange',{oldURL:'old',newURL:'new'});\
         let t=PageTransitionEvent('pageshow',{persisted:true});\
         let b=BeforeUnloadEvent('beforeunload',{returnValue:'leave?'});\
         let r=ProgressEvent('progress',{lengthComputable:true,loaded:3,total:5});\
         p.state.id+':'+p.bubbles+':'+h.oldURL+','+h.newURL+':' +\
         t.persisted+':'+b.returnValue+':'+r.lengthComputable+',' +\
         r.loaded+','+r.total;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "7:true:old,new:true:leave?:true,3,5"
    );
}

#[test]
fn lifecycle_event_dispatch_preserves_progress_fields() {
    let result = eval_with_dom(
        "<progress id='bar'></progress>",
        "let bar=document.getElementById('bar');let seen='';\
         bar.addEventListener('progress',function(e){\
         seen=e.type+':'+e.target.id+':'+e.loaded+':'+e.total;});\
         let ok=bar.dispatchEvent(ProgressEvent('progress',{loaded:4,total:8}));\
         seen+':'+ok;",
    )
    .unwrap();
    assert_eq!(result.value.display(), "progress:bar:4:8:true");
}
