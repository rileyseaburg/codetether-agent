use super::{BrowserPage, DialogKind};
use crate::js::JsValue;

#[path = "dialog_decision_tests.rs"]
mod dialog_decision_tests;

#[test]
fn manual_dialog_records_can_be_cleared() {
    let mut page = BrowserPage::from_html("mem://dialogs", "<main></main>");

    let record = page.record_dialog(DialogKind::Prompt, "Name?", Some("Ada".into()));
    assert_eq!(
        (record.sequence, page.dialogs()[0].default_value.clone()),
        (0, Some("Ada".into()))
    );

    page.clear_dialogs();
    assert!(page.dialogs().is_empty());
}

#[test]
fn run_scripts_captures_alert_dialogs() {
    let mut page = BrowserPage::from_html(
        "mem://dialogs",
        "<script>alert('ready'); window.afterAlert = true;</script>",
    );

    page.run_scripts().unwrap();
    let dialogs = page.dialogs();

    assert_eq!(dialogs.len(), 1);
    assert_eq!(dialogs[0].kind, DialogKind::Alert);
    assert_eq!(dialogs[0].message, "ready");
    assert_eq!(
        page.eval_js("window.afterAlert").unwrap(),
        JsValue::Bool(true)
    );
}
