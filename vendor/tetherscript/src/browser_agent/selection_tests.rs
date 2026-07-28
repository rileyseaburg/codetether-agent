use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn page_select_contents_exposes_selection_text() {
    let mut page = BrowserPage::from_html(
        "https://example.test",
        "<main><p id='note'>Alpha <b>Beta</b></p></main>",
    );
    let report = page.select_contents(&Locator::css("#note")).unwrap();
    assert_eq!(report.action, "select_contents");
    assert_eq!(page.selection_text().unwrap(), "Alpha Beta");
}

#[test]
fn selection_text_reads_focused_input_selection() {
    let mut page = BrowserPage::from_html(
        "https://example.test",
        "<input id='q' value='search'><script>\
         let q=document.getElementById('q'); q.focus(); q.setSelectionRange(1,4);\
         </script>",
    );
    page.run_scripts().unwrap();
    assert_eq!(page.selection_text().unwrap(), "ear");
}
