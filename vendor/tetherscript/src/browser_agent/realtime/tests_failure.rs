//! Realtime failure injection tests.

use crate::browser_agent::{BrowserPage, PageEventKind, RealtimeEventKind};

#[test]
fn websocket_failure_dispatches_error_close_and_logs_reason() {
    let mut page = BrowserPage::from_html("mem://rt", "<p id='out'></p>");
    page.eval_js("let ws=WebSocket('ws://agent'); ws.onerror=function(e){ document.getElementById('out').textContent=e.reason; };")
        .unwrap();
    let id = page.realtime_connections().unwrap()[0].id;

    page.fail_websocket_connection(id, "refused").unwrap();

    let events = page.realtime_events().unwrap();
    assert!(page.session.html.contains(">refused</p>"));
    assert_eq!(events.last().unwrap().event, RealtimeEventKind::Close);
    assert_eq!(events[2].reason.as_deref(), Some("refused"));
    assert!(page.event_log().iter().any(|event| {
        event.kind == PageEventKind::Network && event.message.contains("realtime:error")
    }));
}

#[test]
fn event_source_failure_records_retry_metadata() {
    let mut page = BrowserPage::from_html("mem://rt", "<p id='out'></p>");
    page.eval_js("let es=EventSource('/events'); es.onerror=function(e){ document.getElementById('out').textContent=e.retry + ':' + e.reason; };")
        .unwrap();
    let id = page.realtime_connections().unwrap()[0].id;

    page.fail_event_source_connection(id, "offline", Some(2500))
        .unwrap();

    let connection = page.realtime_connections().unwrap()[0].clone();
    let error = page.realtime_events().unwrap().last().unwrap().clone();
    assert_eq!(connection.retry_ms, Some(2500));
    assert_eq!(error.retry_ms, Some(2500));
    assert!(page.session.html.contains(">2500:offline</p>"));
}
