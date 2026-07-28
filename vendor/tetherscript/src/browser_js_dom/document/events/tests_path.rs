use super::*;

fn eval_nested(script: &str) -> JsValue {
    eval_with_dom(
        "<main id='root'><section id='mid'><button id='leaf'></button></section></main>",
        script,
    )
    .unwrap()
    .value
}

#[test]
fn composed_path_tracks_dispatch_route() {
    let script = "let leaf=document.getElementById('leaf');let seen='';\
        document.getElementById('root').addEventListener('click',function(e){\
        let p=e.composedPath();seen=p[0].id+','+p[1].id+','+p[2].id+','+p[3].nodeType;\
        seen=seen+':'+(e.path.length==p.length)+':'+e.eventPhase+':'+e.target.id+\
        ':'+e.currentTarget.id;});leaf.click();seen;";
    assert_eq!(
        eval_nested(script).display(),
        "leaf,mid,root,9:true:3:leaf:root"
    );
}

#[test]
fn dispatched_event_object_keeps_cancelation_state() {
    let script = "let leaf=document.getElementById('leaf');let e=document.createEvent('Event');\
        e.initEvent('click',true,true);leaf.addEventListener('click',function(x){\
        x.preventDefault();});let ok=leaf.dispatchEvent(e);\
        ok+':'+e.defaultPrevented+':'+e.eventPhase+':'+(e.currentTarget===null)+\
        ':'+e.composedPath()[0].id+':'+e.path[1].id;";
    assert_eq!(eval_nested(script).display(), "false:true:0:true:leaf:mid");
}
