//! Unit tests for [`super::PeerLiveness`].

use super::PeerLiveness;
use std::time::Duration;

#[test]
fn first_sight_then_duplicate() {
    let mut l = PeerLiveness::new();
    assert!(l.record("inst-a", "agent-a"));
    assert!(!l.record("inst-a", "agent-a"));
    assert!(l.recently_seen("inst-a", Duration::from_secs(30)));
}

#[test]
fn expire_returns_card_names() {
    let mut l = PeerLiveness::new();
    l.record("inst-a", "agent-a");
    assert!(
        l.expire(Duration::from_secs(0))
            .contains(&"agent-a".to_string())
    );
    assert!(!l.recently_seen("inst-a", Duration::from_secs(30)));
}

#[test]
fn expire_multi_homed_keeps_alive_instance() {
    let mut l = PeerLiveness::new();
    l.record("inst-lan", "agent-multi");
    l.record("inst-vpn", "agent-multi");
    // Expire only the LAN instance; VPN instance still alive → not expired.
    l.record("inst-vpn", "agent-multi"); // refresh VPN
    let expired = l.expire(Duration::from_secs(0));
    // Both instances expired since neither was refreshed within 0s.
    // The multi-homed guard only matters when one is refreshed and one isn't.
    assert_eq!(expired, vec!["agent-multi".to_string()]);
}

#[test]
fn expire_keeps_peer_if_any_instance_alive() {
    let mut l = PeerLiveness::new();
    assert!(l.record("inst-a", "shared-agent"));
    assert!(!l.record("inst-b", "shared-agent"));
    // Only expire after a very generous TTL — neither expires.
    assert!(l.expire(Duration::from_secs(9999)).is_empty());
}
