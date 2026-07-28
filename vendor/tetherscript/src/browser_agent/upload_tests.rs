use super::{BrowserPage, FilePayload, Locator};
use crate::js::JsValue;

#[test]
fn set_single_file_metadata_and_events() {
    let html = "<input id='u' type='file'><script>let log='';let u=document.getElementById('u');u.addEventListener('input',function(){log=log+'i'});u.addEventListener('change',function(){log=log+'c'});</script>";
    let mut page = BrowserPage::from_html("mem://upload", html);
    page.run_scripts().unwrap();

    let files = vec![FilePayload::new("note.txt", "text/plain", b"abc".to_vec())];
    let report = page.set_input_files(&Locator::css("#u"), files).unwrap();
    let value = page.eval_js("let u=document.getElementById('u');u.getAttribute('data-agent-files')+':'+u.getAttribute('data-agent-file-count')+':'+u.value+':'+log").unwrap();

    assert_eq!(report.action, "set_input_files");
    let expected =
        "[{\"name\":\"note.txt\",\"type\":\"text/plain\",\"size\":3}]:1:C:\\fakepath\\note.txt:ic";
    assert_eq!(value, JsValue::String(expected.into()));
}

#[test]
fn set_multiple_files_when_multiple_attribute_is_present() {
    let mut page = BrowserPage::from_html("mem://upload", "<input id='u' type='file' multiple>");

    let files = vec![
        FilePayload::new("a.txt", "text/plain", b"a".to_vec()),
        FilePayload::new("b.bin", "application/octet-stream", vec![1, 2]),
    ];
    page.set_input_files(&Locator::css("#u"), files).unwrap();
    let metadata = page
        .eval_js("document.getElementById('u').getAttribute('data-agent-files')")
        .unwrap();

    assert!(page.session.html.contains("data-agent-file-count=\"2\""));
    let expected = "[{\"name\":\"a.txt\",\"type\":\"text/plain\",\"size\":1},{\"name\":\"b.bin\",\"type\":\"application/octet-stream\",\"size\":2}]";
    assert_eq!(metadata, JsValue::String(expected.into()));
}

#[test]
fn set_input_files_rejects_wrong_element() {
    let mut page = BrowserPage::from_html("mem://upload", "<button id='u'>Upload</button>");

    let err = page
        .set_input_files(&Locator::css("#u"), vec![])
        .unwrap_err();

    assert!(err.contains("input[type=file]"));
}

#[test]
fn set_input_files_rejects_disabled_input() {
    let mut page = BrowserPage::from_html("mem://upload", "<input id='u' type='file' disabled>");

    let err = page
        .set_input_files(&Locator::css("#u"), vec![])
        .unwrap_err();

    assert!(err.contains("disabled"));
}
