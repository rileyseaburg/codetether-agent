use super::*;

#[test]
fn fullscreen_api_updates_state_and_fires_events() {
    let result = eval_with_dom(
        "<div id='box'></div>",
        "function fs(){if(document.fullscreenElement){return document.fullscreenElement.id;}return 'null';}\
         function on_box(e){seen=seen+'E:'+fs()+':'+e.target.id+'|';}\
         function on_doc(e){seen=seen+'D:'+fs()+':'+e.target.id+'|';}\
         let box=document.getElementById('box');\
         let seen='';\
         box.addEventListener('fullscreenchange', on_box);\
         document.addEventListener('fullscreenchange', on_doc);\
         let p=box.requestFullscreen();\
         box.requestFullscreen();\
         let shape=typeof box.requestFullscreen+':'+typeof document.exitFullscreen+':'+typeof p.then+':'+document.fullscreenEnabled;\
         document.exitFullscreen();\
         shape+';'+seen+';'+document.fullscreenElement;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "function:function:function:true;E:box:box|D:box:box|E:null:box|D:null:box|;null"
                .into()
        )
    );
}
