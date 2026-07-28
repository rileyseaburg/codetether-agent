use super::super::*;

#[test]
fn device_media_event_defaults_match_browser_probes() {
    let result = eval_with_dom(
        "",
        "let q=MediaQueryListEvent('change');\
         let o=DeviceOrientationEvent('deviceorientation');\
         let m=DeviceMotionEvent('devicemotion');\
         typeof MediaQueryListEvent+':'+typeof DeviceOrientationEvent+':' +\
         typeof DeviceMotionEvent+':'+q.matches+','+q.media+':' +\
         (o.alpha===null)+','+(o.beta===null)+','+(o.gamma===null)+',' +\
         o.absolute+':'+(m.acceleration===null)+',' +\
         (m.accelerationIncludingGravity===null)+','+(m.rotationRate===null)+\
         ','+m.interval+':'+typeof m.preventDefault+':'+m.defaultPrevented;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "function:function:function:false,:true,true,true,false:true,true,true,0:function:false"
    );
}
