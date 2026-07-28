use super::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn hover_dispatches_hover_events() {
    let html = "<button id='go'>Go</button><script>window.hoverLog=''; let b=document.getElementById('go'); b.addEventListener('mouseover', function(){ window.hoverLog=window.hoverLog+'over>'; }); b.addEventListener('mouseenter', function(){ window.hoverLog=window.hoverLog+'enter>'; }); b.addEventListener('mousemove', function(){ window.hoverLog=window.hoverLog+'move'; });</script>";
    let mut page = BrowserPage::from_html("mem://hover", html);
    page.run_scripts().unwrap();

    let report = page.hover(&Locator::css("#go")).unwrap();
    let value = page.eval_js("window.hoverLog").unwrap();

    assert_eq!(report.action, "hover");
    assert_eq!(value, JsValue::String("over>enter>move".into()));
}

#[test]
fn double_click_dispatches_full_sequence() {
    let html = "<button id='go'>Go</button><script>window.mouseLog=''; let b=document.getElementById('go'); function add(e){ window.mouseLog=window.mouseLog+e.type+'>'; } b.addEventListener('mousedown', add); b.addEventListener('mouseup', add); b.addEventListener('click', add); b.addEventListener('dblclick', add);</script>";
    let mut page = BrowserPage::from_html("mem://dbl", html);
    page.run_scripts().unwrap();

    page.double_click(&Locator::css("#go")).unwrap();
    let value = page.eval_js("window.mouseLog").unwrap();

    assert_eq!(
        value,
        JsValue::String("mousedown>mouseup>click>mousedown>mouseup>click>dblclick>".into())
    );
}

#[test]
fn mouse_down_up_order_is_deterministic() {
    let html = "<button id='go'>Go</button><script>window.mouseLog=''; let b=document.getElementById('go'); b.addEventListener('mousedown', function(){ window.mouseLog=window.mouseLog+'down>'; }); b.addEventListener('mouseup', function(){ window.mouseLog=window.mouseLog+'up'; });</script>";
    let mut page = BrowserPage::from_html("mem://down-up", html);
    page.run_scripts().unwrap();

    page.mouse_down(&Locator::css("#go")).unwrap();
    page.mouse_up(&Locator::css("#go")).unwrap();

    assert_eq!(
        page.eval_js("window.mouseLog").unwrap(),
        JsValue::String("down>up".into())
    );
}

#[test]
fn pointer_actions_reuse_actionability_errors() {
    let mut disabled =
        BrowserPage::from_html("mem://disabled", "<button id='go' disabled>Go</button>");
    let err = disabled.mouse_down(&Locator::css("#go")).unwrap_err();
    assert!(err.contains("enabled"));

    let html = "<button id='target' style='width:20px;height:4px'>Save</button><div id='cover' style='position:absolute;left:0;top:0;width:20px;height:4px'></div>";
    let mut covered = BrowserPage::from_html("mem://covered", html);
    let err = covered.hover(&Locator::css("#target")).unwrap_err();
    assert!(err.contains("receives_pointer"));
    assert!(err.contains("div#cover"));
}
