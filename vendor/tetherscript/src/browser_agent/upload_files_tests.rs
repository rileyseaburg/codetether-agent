use super::{BrowserPage, FilePayload, Locator};
use crate::js::JsValue;

#[test]
fn set_input_files_exposes_files_to_fresh_lookup() {
    let mut page = BrowserPage::from_html("mem://upload", "<input id='u' type='file' multiple>");
    let files = vec![
        FilePayload::new("a.txt", "text/plain", b"a".to_vec()),
        FilePayload::new("b.bin", "application/octet-stream", vec![1, 2]),
    ];
    page.set_input_files(&Locator::css("#u"), files).unwrap();
    let value = page.eval_js("let f=document.getElementById('u').files; f.length+':'+f[0].name+':'+f.item(1).name+':'+f[1].size").unwrap();
    assert_eq!(value, JsValue::String("2:a.txt:b.bin:2".into()));
}
