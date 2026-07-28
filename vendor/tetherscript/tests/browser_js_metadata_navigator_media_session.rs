use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn navigator_media_session_fields_are_writable() {
    let result = eval_with_dom(
        "<main></main>",
        "let m=navigator.mediaSession;\
         let before=typeof m+':'+m.metadata+':'+m.playbackState;\
         m.metadata={title:'Song',artist:'Ada'}; m.playbackState='playing';\
         [before,m.metadata.title,m.metadata.artist,m.playbackState].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("object:null:none|Song|Ada|playing".into())
    );
}

#[test]
fn navigator_media_session_records_handlers_and_position() {
    let result = eval_with_dom(
        "<main></main>",
        "let m=navigator.mediaSession;\
         let added=''+m.setActionHandler('play',function(){return 'ok';});\
         let seen=typeof m.__actionHandlers.play+':'+m.__actionHandlers.play();\
         let removed=''+m.setActionHandler('play',null);\
         let gone=typeof m.__actionHandlers.play;\
         let state={duration:120,position:7,playbackRate:1};\
         let positioned=''+m.setPositionState(state); state.position=99;\
         let saved=[m.__positionState.duration,m.__positionState.position,\
         m.__positionState.playbackRate].join(':');\
         let cleared=''+m.setPositionState();\
         [added,seen,removed,gone,positioned,saved,cleared,\
         typeof m.__positionState].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "undefined|function:ok|undefined|undefined|undefined|120:7:1|undefined|undefined"
                .into()
        )
    );
}
