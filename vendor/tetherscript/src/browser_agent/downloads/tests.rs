use super::{DownloadRecord, DownloadStatus};
use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn manual_download_records_are_ordered_and_clearable() {
    let mut page = BrowserPage::from_html("mem://downloads", "<main></main>");
    let record = DownloadRecord::completed(
        "https://example.test/report.txt",
        "report.txt",
        "text/plain",
        b"hello".to_vec(),
    );

    page.record_download(record.clone());

    assert_eq!(page.downloads(), &[record]);
    assert_eq!(page.downloads()[0].byte_len, 5);
    assert_eq!(page.downloads()[0].status, DownloadStatus::Completed);
    page.clear_downloads();
    assert!(page.downloads().is_empty());
}

#[test]
fn anchor_download_click_records_suggested_filename() {
    let mut page = BrowserPage::from_html(
        "https://example.test",
        "<a id='dl' href='https://example.test/file.bin' download='out.bin'>Save</a>",
    );

    page.click(&Locator::css("#dl")).unwrap();

    assert_eq!(page.downloads().len(), 1);
    assert_eq!(page.downloads()[0].url, "https://example.test/file.bin");
    assert_eq!(page.downloads()[0].suggested_filename, "out.bin");
}

#[test]
fn normal_anchor_click_does_not_record_download() {
    let mut page = BrowserPage::from_html(
        "https://example.test",
        "<a id='next' href='https://example.test/next'>Next</a>",
    );

    page.click(&Locator::css("#next")).unwrap();

    assert!(page.downloads().is_empty());
}
