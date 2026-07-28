use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn viewport_globals_and_visibility_defaults_are_exposed() {
    let script = "window.innerWidth + ':' + innerWidth + ':' + innerHeight + ':'\
        + screen.width + ':' + screen.availHeight + ':' + devicePixelRatio + ':'\
        + document.hidden + ':' + document.visibilityState;";
    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("80:80:24:80:24:1:false:visible".into())
    );
}

#[test]
fn match_media_uses_deterministic_viewport_and_media_defaults() {
    let script = "let a=matchMedia('screen and (min-width: 80px)');\
        let b=window.matchMedia('(prefers-color-scheme: dark)');\
        a.matches + ':' + b.matches + ':' + a.media;";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("true:false:screen and (min-width: 80px)".into())
    );
}

#[test]
fn match_media_dispatches_change_listeners() {
    let script = "let m=matchMedia('(max-width: 79px)'); let out='';\
        let f=function(){out=out+'x';}; m.addListener(f); m.removeListener(f);\
        m.addEventListener('change', function(e){out=out+'E'+e.matches;});\
        m.onchange=function(e){out=out+'O'+e.media;};\
        m.dispatchEvent({type:'change'}); out;";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("EfalseO(max-width: 79px)".into())
    );
}
