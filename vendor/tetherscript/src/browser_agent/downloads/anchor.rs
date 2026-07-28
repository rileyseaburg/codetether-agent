//! Anchor download click helpers.

use crate::browser_agent::downloads::{filename, DownloadRecord};
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::js::JsValue;

pub(crate) fn is_anchor_download(resolved: &Resolved) -> bool {
    resolved.dom.element.tag.eq_ignore_ascii_case("a")
        && resolved.dom.element.attrs.contains_key("download")
        && resolved.dom.element.attrs.contains_key("href")
}

pub(crate) fn click_script(path: &[usize]) -> String {
    format!("let n={}; n.click()", node(path))
}

pub(crate) fn record_anchor_download(
    page: &mut BrowserPage,
    resolved: &Resolved,
    click_result: &JsValue,
) {
    if !is_anchor_download(resolved) || click_result == &JsValue::Bool(false) {
        return;
    }
    let element = &resolved.dom.element;
    let href = element.attrs.get("href").cloned().unwrap_or_default();
    let filename = filename::suggested(element.attrs.get("download"), &href);
    let mime = element
        .attrs
        .get("type")
        .cloned()
        .unwrap_or_else(|| "application/octet-stream".into());
    page.record_download(DownloadRecord::completed(
        href,
        filename,
        mime,
        Vec::<u8>::new(),
    ));
}

fn node(path: &[usize]) -> String {
    path.iter()
        .fold(String::from("document"), |mut script, index| {
            script.push_str(&format!(".childNodes[{index}]"));
            script
        })
}
