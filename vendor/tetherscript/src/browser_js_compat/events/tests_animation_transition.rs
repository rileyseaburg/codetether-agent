use super::*;

#[test]
fn animation_transition_rejection_defaults() {
    let result = eval_with_dom(
        "",
        "let a=AnimationEvent('animationstart');\
         let t=TransitionEvent('transitionend');\
         let r=PromiseRejectionEvent('unhandledrejection');\
         typeof AnimationEvent+':'+typeof TransitionEvent+':' +\
         typeof PromiseRejectionEvent+':'+a.animationName+',' +\
         a.elapsedTime+','+a.pseudoElement+':'+t.propertyName+',' +\
         t.elapsedTime+','+t.pseudoElement+':'+(r.promise===null)+',' +\
         (r.reason===null)+':'+typeof a.preventDefault+':' +\
         a.defaultPrevented;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "function:function:function:,0,:,0,:true,true:function:false"
    );
}

#[test]
fn animation_transition_rejection_read_init_fields() {
    let result = eval_with_dom(
        "",
        "let p={state:'rejected'};let reason={message:'bad'};\
         let a=AnimationEvent('animationend',{animationName:'fade',\
         elapsedTime:1.25,pseudoElement:'::before',bubbles:true});\
         let t=TransitionEvent('transitionend',{propertyName:'opacity',\
         elapsedTime:0.5,pseudoElement:'::after'});\
         let r=PromiseRejectionEvent('unhandledrejection',{promise:p,\
         reason:reason});a.animationName+':'+a.elapsedTime+':' +\
         a.pseudoElement+':'+a.bubbles+'|'+t.propertyName+':' +\
         t.elapsedTime+':'+t.pseudoElement+'|'+r.promise.state+':' +\
         r.reason.message;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "fade:1.25:::before:true|opacity:0.5:::after|rejected:bad"
    );
}
