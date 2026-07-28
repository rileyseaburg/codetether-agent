//! Focused navigation lifecycle tests for the JS host.

use super::super::{eval_with_dom, BrowserJsRuntime, BrowserJsState};
use crate::js::JsValue;

#[test]
fn runtime_dispatches_beforeunload_and_unload_handlers() {
    let html = "<script>window.addEventListener('beforeunload',function(){console.log('B');});window.onunload=function(){console.log('U');};</script>";
    let mut runtime = BrowserJsRuntime::new(html, BrowserJsState::default()).unwrap();
    runtime.run_scripts().unwrap();

    let before = runtime.dispatch_beforeunload().unwrap();
    let unload = runtime.dispatch_unload().unwrap();

    assert_eq!(before.console, vec!["B".to_string()]);
    assert_eq!(unload.console, vec!["U".to_string()]);
}

#[test]
fn runtime_dispatches_page_and_visibility_events() {
    let html = "<script>window.onpageshow=function(e){console.log('S:'+e.type+':'+(e.target===window));};window.addEventListener('pagehide',function(e){console.log('H:'+e.type);});window.onvisibilitychange=function(e){console.log('V:'+e.type);};</script>";
    let mut runtime = BrowserJsRuntime::new(html, BrowserJsState::default()).unwrap();
    runtime.run_scripts().unwrap();

    let show = runtime.dispatch_pageshow().unwrap();
    let hide = runtime.dispatch_pagehide().unwrap();
    let visibility = runtime.dispatch_visibilitychange().unwrap();

    assert_eq!(show.console, vec!["S:pageshow:true".to_string()]);
    assert_eq!(hide.console, vec!["H:pagehide".to_string()]);
    assert_eq!(visibility.console, vec!["V:visibilitychange".to_string()]);
}

#[test]
fn history_back_dispatches_popstate_before_hashchange() {
    let script = "let seen='';window.addEventListener('popstate',function(e){seen=seen+'P'+e.state.page;});window.addEventListener('hashchange',function(e){seen=seen+'H'+e.oldURL+'>'+e.newURL;});history.pushState({page:1},'','/first#one');history.pushState({page:2},'','/first#two');history.back();seen;";

    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("P1H/first#two>/first#one".into())
    );
}
