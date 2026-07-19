//! Emits the signed Forgejo principal binding used for durable routing.

pub(super) fn push_forgejo_trailers(trailers: &mut Vec<(&'static str, String)>) {
    if let Some((host, login, slot)) = super::forgejo_identity::configured_principal() {
        trailers.push(("CodeTether-Forgejo-Host", host));
        trailers.push(("CodeTether-Forgejo-Login", login));
        trailers.push(("CodeTether-Agent-Slot", slot));
    }
}
