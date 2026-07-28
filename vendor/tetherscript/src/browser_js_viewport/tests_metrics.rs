use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn deterministic_window_metrics_are_exposed() {
    let script = "[
        window.outerWidth, window.outerHeight, window.screenX, window.screenY,
        window.screenLeft, window.screenTop
    ].join(':');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(result.value, JsValue::String("80:24:0:0:0:0".into()));
}

#[test]
fn visual_viewport_exposes_deterministic_metrics_and_noop_events() {
    let script = "let v=window.visualViewport;\
        let resize=v.dispatchEvent({type:'resize'});\
        v.addEventListener('scroll', function(){});\
        v.removeEventListener('scroll', function(){});\
        [v.width,v.height,v.scale,v.offsetLeft,v.offsetTop,\
        v.pageLeft,v.pageTop,resize].join(':');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(result.value, JsValue::String("80:24:1:0:0:0:0:true".into()));
}

#[test]
fn resize_to_leaves_scoped_outer_and_visual_metrics_static() {
    let script = "resizeTo(120,40);\
        [window.innerWidth,window.outerWidth,\
        window.visualViewport.width,window.visualViewport.height].join(':');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(result.value, JsValue::String("120:80:80:24".into()));
}

#[test]
fn screen_orientation_is_deterministic() {
    let script = "let o=screen.orientation;\
        let a=o.addEventListener('change',function(){});\
        let r=o.dispatchEvent({type:'change'});\
        let m=o.removeEventListener('change',function(){});\
        [window.orientation,o.type,o.angle,o.onchange,a,r,m].join(':');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("0:landscape-primary:0:::true:".into())
    );
}
