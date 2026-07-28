use super::*;

#[test]
fn mutation_observer_checkpoint_runs_callback_microtasks() {
    let result = eval_with_dom(
        "<div id='target'></div>",
        "let log=''; let target=document.getElementById('target');\
         let obs=MutationObserver(function(records){\
         log=log+'O'+records.length+records[0].addedNodes.length;\
         queueMicrotask(function(){ log=log+'M'; console.log(log); }); });\
         obs.observe(target,{childList:true});\
         queueMicrotask(function(){ log=log+'Q';\
         target.appendChild(document.createElement('span')); });\
         log=log+'S'; 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["SQO11M".to_string()]);
}

#[test]
fn mutation_observer_subtree_records_target_and_old_value() {
    let result = eval_with_dom(
        "<div id='parent'><span id='child'></span></div>",
        "let parent=document.getElementById('parent');\
         let child=document.getElementById('child');\
         child.setAttribute('data-x','old');\
         let obs=MutationObserver(function(records){\
         console.log(records[0].target.id + ':' + records[0].attributeName + ':' + records[0].oldValue); });\
         obs.observe(parent,{subtree:true,attributes:true,attributeOldValue:true});\
         child.setAttribute('data-x','new'); 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["child:data-x:old".to_string()]);
}

#[test]
fn intersection_observer_entry_uses_element_geometry() {
    let result = eval_with_dom(
        "<style>#box { width: 12px; height: 5px }</style><div id='box'></div>",
        "let box=document.getElementById('box');\
         let obs=IntersectionObserver(function(entries){\
         let rect=entries[0].boundingClientRect;\
         console.log(rect.width + ':' + rect.height + ':' + entries[0].intersectionRect.width); });\
         obs.observe(box); 'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["12:5:12".to_string()]);
}
