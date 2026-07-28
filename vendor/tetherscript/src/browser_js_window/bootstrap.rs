//! JavaScript bootstrap for window scroll and resize compatibility.

pub(super) const SOURCE: &str = r#"
var onpagehide=window.onpagehide;
var onpageshow=window.onpageshow;
var onvisibilitychange=window.onvisibilitychange;
var ononline=window.ononline;
var onoffline=window.onoffline;
var onresize=window.onresize;
var onscroll=window.onscroll;
var DOMException=window.DOMException;
var Image=window.Image;var Option=window.Option;
var DOMPoint=window.DOMPoint;var DOMRect=window.DOMRect;
var StorageEvent=window.StorageEvent;var ClipboardEvent=window.ClipboardEvent;var ClipboardItem=window.ClipboardItem;var Notification=window.Notification;
var MediaQueryListEvent=window.MediaQueryListEvent;var DeviceOrientationEvent=window.DeviceOrientationEvent;var DeviceMotionEvent=window.DeviceMotionEvent;
var MessageEvent=window.MessageEvent;var ErrorEvent=window.ErrorEvent;var CloseEvent=window.CloseEvent;
var DragEvent=window.DragEvent;var CompositionEvent=window.CompositionEvent;var TouchEvent=window.TouchEvent;var DataTransfer=window.DataTransfer;
var PopStateEvent=window.PopStateEvent;var HashChangeEvent=window.HashChangeEvent;
var PageTransitionEvent=window.PageTransitionEvent;var BeforeUnloadEvent=window.BeforeUnloadEvent;var ProgressEvent=window.ProgressEvent;
var AnimationEvent=window.AnimationEvent;var TransitionEvent=window.TransitionEvent;var PromiseRejectionEvent=window.PromiseRejectionEvent;
var open=window.open;var close=window.close;var print=window.print;var stop=window.stop;var focus=window.focus;var blur=window.blur;
var showOpenFilePicker=window.showOpenFilePicker;var showSaveFilePicker=window.showSaveFilePicker;var showDirectoryPicker=window.showDirectoryPicker;
function __tsSetScroll(x,y){x=x*1;y=y*1;window.scrollX=x;window.scrollY=y;window.pageXOffset=x;window.pageYOffset=y;scrollX=x;scrollY=y;pageXOffset=x;pageYOffset=y;window.__tsDispatchScroll();return undefined;}
function scrollTo(x,y){return __tsSetScroll(x,y);}
function scroll(x,y){return __tsSetScroll(x,y);}
function scrollBy(dx,dy){return __tsSetScroll(scrollX+dx,scrollY+dy);}
function resizeTo(width,height){width=width*1;height=height*1;window.innerWidth=width;window.innerHeight=height;innerWidth=width;innerHeight=height;window.__tsDispatchResize();return undefined;}
window.scrollTo=scrollTo;window.scrollBy=scrollBy;window.scroll=scroll;window.resizeTo=resizeTo;
"#;
