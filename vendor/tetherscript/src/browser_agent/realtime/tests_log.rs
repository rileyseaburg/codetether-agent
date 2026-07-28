//! Realtime lifecycle logging tests.

use crate::browser_agent::{BrowserPage, RealtimeEventKind};

#[test]
fn websocket_lifecycle_records_send_and_network_events() {
    let mut page = BrowserPage::from_html("mem://rt", "<main></main>");
    page.eval_js("let ws=WebSocket('ws://agent'); ws.onopen=function(){ ws.send('ping'); ws.close(1000,'done'); };")
        .unwrap();

    let events = page.realtime_events().unwrap();
    let kinds: Vec<_> = events.iter().map(|event| event.event).collect();
    assert_eq!(
        kinds,
        vec![
            RealtimeEventKind::Connect,
            RealtimeEventKind::Open,
            RealtimeEventKind::Send,
            RealtimeEventKind::Close,
        ]
    );
    assert_eq!(events[2].data.as_deref(), Some("ping"));
    assert_eq!(events[3].code, Some(1000));
    assert!(page
        .network_events()
        .iter()
        .any(|event| event.route_result.as_deref() == Some("realtime:send")));
}
