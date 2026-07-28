//! Conversion helpers from the JS realtime host.

use super::{RealtimeConnection, RealtimeEvent, RealtimeEventKind, RealtimeKind};
use crate::browser_js::{
    BrowserJsRealtimeConnection, BrowserJsRealtimeEvent, BrowserJsRealtimeEventKind,
    BrowserJsRealtimeKind,
};

pub(crate) fn connection_from_js(connection: BrowserJsRealtimeConnection) -> RealtimeConnection {
    RealtimeConnection {
        id: connection.id,
        kind: kind_from_js(connection.kind),
        url: connection.url,
        ready_state: connection.ready_state,
        retry_ms: connection.retry_ms,
    }
}

pub(crate) fn event_from_js(event: BrowserJsRealtimeEvent) -> RealtimeEvent {
    RealtimeEvent {
        sequence: event.sequence,
        connection_id: event.connection_id,
        event: event_kind_from_js(event.event),
        connection_kind: kind_from_js(event.connection_kind),
        url: event.url,
        ready_state: event.ready_state,
        data: event.data,
        code: event.code,
        reason: event.reason,
        retry_ms: event.retry_ms,
    }
}

fn kind_from_js(kind: BrowserJsRealtimeKind) -> RealtimeKind {
    match kind {
        BrowserJsRealtimeKind::WebSocket => RealtimeKind::WebSocket,
        BrowserJsRealtimeKind::EventSource => RealtimeKind::EventSource,
    }
}

fn event_kind_from_js(event: BrowserJsRealtimeEventKind) -> RealtimeEventKind {
    match event {
        BrowserJsRealtimeEventKind::Connect => RealtimeEventKind::Connect,
        BrowserJsRealtimeEventKind::Open => RealtimeEventKind::Open,
        BrowserJsRealtimeEventKind::Send => RealtimeEventKind::Send,
        BrowserJsRealtimeEventKind::Receive => RealtimeEventKind::Receive,
        BrowserJsRealtimeEventKind::Close => RealtimeEventKind::Close,
        BrowserJsRealtimeEventKind::Error => RealtimeEventKind::Error,
    }
}
