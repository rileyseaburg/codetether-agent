use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn navigator_media_capabilities_decoding_info_resolves_deterministically() {
    let value = eval(
        "let seen='';\
         let config={type:'file',video:{contentType:'video/mp4; codecs=\"avc1\"'}};\
         let p=navigator.mediaCapabilities.decodingInfo(config);\
         p.then(function(i){seen=[i.supported,i.smooth,i.powerEfficient,\
         i.type,i.contentType].join('|');});\
         [typeof navigator.mediaCapabilities,typeof navigator.mediaCapabilities.decodingInfo,\
         p.__promise_state,p.__promise_value.supported,p.__promise_value.type,\
         p.__promise_value.contentType,seen].join(';');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "object;function;fulfilled;true;file;video/mp4; codecs=\"avc1\";\
             true|true|true|file|video/mp4; codecs=\"avc1\""
                .into()
        )
    );
}

#[test]
fn navigator_media_capabilities_encoding_info_accepts_audio_config() {
    let value = eval(
        "let p=navigator.mediaCapabilities.encodingInfo({type:'record',\
         audio:{contentType:'audio/webm; codecs=\"opus\"'}});\
         [p.__promise_value.supported,p.__promise_value.smooth,\
         p.__promise_value.powerEfficient,p.__promise_value.type,\
         p.__promise_value.contentType].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String("true|true|true|record|audio/webm; codecs=\"opus\"".into())
    );
}
