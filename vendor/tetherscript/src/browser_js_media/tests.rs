use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn media_load_sets_metadata_and_events() {
    let html = "<video id='v' src='clip.mp4' duration='12'></video>";
    let js = "let v=document.getElementById('v');let e=[];\
        ['loadstart','loadedmetadata','canplay'].forEach(function(t){\
        v.addEventListener(t,function(x){e.push(x.type);});});\
        v.load();v.currentSrc+'|'+v.duration+'|'+v.readyState+'|'+e.join(',');";
    assert_eq!(
        eval_with_dom(html, js).unwrap().value,
        JsValue::String("clip.mp4|12|4|loadstart,loadedmetadata,canplay".into())
    );
}

#[test]
fn media_play_pause_updates_state_and_thenable() {
    let html = "<audio id='a' src='song.mp3'></audio>";
    let js = "let a=document.getElementById('a');let e=[];let seen='';\
        ['play','playing','pause'].forEach(function(t){a.addEventListener(t,function(x){e.push(x.type);});});\
        a.play().then(function(){seen=a.paused+':'+a.ended;});a.pause();\
        seen+'|'+a.paused+'|'+e.join(',');";
    assert_eq!(
        eval_with_dom(html, js).unwrap().value,
        JsValue::String("false:false|true|play,playing,pause".into())
    );
}

#[test]
fn media_setters_canplay_and_error_are_deterministic() {
    let html = "<video id='v'></video>";
    let js = "let v=document.getElementById('v');let caught='';let events=[];\
        v.onerror=function(){events.push('error');};\
        v.play().catch(function(e){caught=e.code+':'+e.message;});\
        v.src='movie.webm';v.volume=2;v.muted=true;v.playbackRate=1.5;\
        v.canPlayType('video/webm')+'|'+v.src+'|'+v.volume+'|'+v.muted+'|'+\
        v.playbackRate+'|'+caught+'|'+events.join(',');";
    assert_eq!(
        eval_with_dom(html, js).unwrap().value,
        JsValue::String("probably|movie.webm|1|true|1.5|4:media source unavailable|error".into())
    );
}
