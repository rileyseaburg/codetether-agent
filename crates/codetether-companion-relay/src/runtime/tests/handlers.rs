//! Exercise private reply handlers independently of the current route whitelist.
use super::handler_fixture::{paired, value};
use axum::{body::Body, http::HeaderMap};
use serde_json::json;

#[tokio::test]
async fn direct_owner_reply_and_device_ack_preserve_their_contract() {
    let (relay, id, token) = paired();
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let queued = crate::owner::session(
        &relay,
        "POST",
        &id,
        Some("reply"),
        &headers,
        Body::from(json!({"text":"hello"}).to_string()),
    )
    .await
    .unwrap();
    assert_eq!(queued.status(), 202);
    let queued = value(queued).await;
    let auth = format!("Bearer {token}");
    let polled = crate::device::session(
        &relay,
        "commands",
        &id,
        Some(&auth),
        &headers,
        Body::empty(),
    )
    .await
    .unwrap();
    assert_eq!(value(polled).await["reply"]["text"], "hello");
    let typed = crate::device::session(
        &relay,
        "typed",
        &id,
        Some(&auth),
        &headers,
        Body::from(json!({"reply_id":queued["reply_id"]}).to_string()),
    )
    .await
    .unwrap();
    assert_eq!(typed.status(), 200);
    assert_eq!(value(typed).await, json!({"typed":true}));
    let mut state = relay.lock();
    assert!(state.live(&id, crate::now()).unwrap().reply.is_none());
}
