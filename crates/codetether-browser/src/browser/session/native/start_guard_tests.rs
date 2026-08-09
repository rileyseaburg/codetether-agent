use crate::browser::request::StartRequest;

fn request() -> StartRequest {
    StartRequest {
        headless: true,
        executable_path: None,
        user_data_dir: None,
        ws_url: None,
    }
}

#[test]
fn plain_native_start_is_allowed() {
    assert!(super::reject_real_browser_request(&request()).is_ok());
}

#[test]
fn devtools_endpoint_is_rejected_rather_than_ignored() {
    let mut start = request();
    start.ws_url = Some("ws://localhost:9222/devtools/browser/x".into());

    let error = super::reject_real_browser_request(&start).expect_err("must reject");

    assert!(error.to_string().contains("ws_url"));
}

#[test]
fn browser_binary_is_rejected_rather_than_ignored() {
    let mut start = request();
    start.executable_path = Some("/usr/bin/chromium".into());

    let error = super::reject_real_browser_request(&start).expect_err("must reject");

    assert!(error.to_string().contains("executable_path"));
}
