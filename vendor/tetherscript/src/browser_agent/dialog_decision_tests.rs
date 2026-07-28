use crate::browser_agent::{BrowserPage, DialogKind};
use crate::js::JsValue;

#[test]
fn queued_confirm_decisions_set_record_state() {
    let mut page = BrowserPage::from_html("mem://dialogs", "<main></main>");

    page.accept_next_dialog();
    let accepted = page.eval_js("window.confirm('continue?')").unwrap();
    page.dismiss_next_dialog();
    let dismissed = page.eval_js("window.confirm('stop?')").unwrap();
    let dialogs = page.dialogs();

    assert_eq!(
        (accepted, dismissed),
        (JsValue::Bool(true), JsValue::Bool(false))
    );
    assert_eq!(dialogs[0].accepted, Some(true));
    assert_eq!(dialogs[1].accepted, Some(false));
}

#[test]
fn queued_prompt_decision_returns_response() {
    let mut page = BrowserPage::from_html("mem://dialogs", "<main></main>");

    page.accept_next_prompt("Grace");
    let value = page.eval_js("window.prompt('Name?', 'Ada')").unwrap();
    let dialog = &page.dialogs()[0];

    assert_eq!(value, JsValue::String("Grace".into()));
    assert_eq!(dialog.kind, DialogKind::Prompt);
    assert_eq!(dialog.default_value, Some("Ada".into()));
    assert_eq!(dialog.response, Some("Grace".into()));
}
