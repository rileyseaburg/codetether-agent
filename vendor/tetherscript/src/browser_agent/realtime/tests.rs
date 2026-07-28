//! Realtime page API tests.

use crate::browser_agent::{BrowserPage, RealtimeKind};

#[test]
fn websocket_send_records_outbound_message() {
    let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    page.eval_js("let ws=WebSocket('ws://agent'); ws.onopen=function(){ ws.send('ping'); };")
        .unwrap();

    let messages = page.realtime_outbound_messages().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].url, "ws://agent");
    assert_eq!(messages[0].data, "ping");
}

#[test]
fn websocket_message_injection_dispatches_onmessage() {
    let mut page = BrowserPage::from_html("mem://rt", "<main><p id='out'></p></main>");
    page.eval_js(
        "let ws=WebSocket('ws://agent'); ws.onmessage=function(e){ document.getElementById('out').textContent=e.data; };",
    )
    .unwrap();
    let connection = page.realtime_connections().unwrap()[0].clone();

    page.inject_websocket_message(connection.id, "hello")
        .unwrap();

    assert_eq!(connection.kind, RealtimeKind::WebSocket);
    assert!(page.session.html.contains("<p id=\"out\">hello</p>"));
}

#[test]
fn event_source_message_injection_dispatches_onmessage() {
    let mut page = BrowserPage::from_html("mem://rt", "<main><p id='out'></p></main>");
    page.eval_js(
        "let es=EventSource('/events'); es.onmessage=function(e){ document.getElementById('out').textContent=e.data; };",
    )
    .unwrap();
    let connection = page.realtime_connections().unwrap()[0].clone();

    page.inject_event_source_message(connection.id, "stream")
        .unwrap();

    assert_eq!(connection.kind, RealtimeKind::EventSource);
    assert!(page.session.html.contains("<p id=\"out\">stream</p>"));
}
