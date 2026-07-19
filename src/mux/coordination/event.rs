//! Advisory bus visibility for decisions already enforced by the mux server.

use crate::bus::BusMessage;
use crate::mux::lease::CoordinationReply;

pub(super) fn publish(action: &str, owner: &str, reply: &CoordinationReply) {
    if quiet(reply) {
        return;
    }
    let Some(bus) = crate::bus::global::global() else {
        return;
    };
    let Ok(value) = serde_json::to_value(reply) else {
        return;
    };
    let key = format!("mux/coordination/{owner}");
    bus.handle(owner).send(
        key.clone(),
        BusMessage::SharedResult {
            key,
            value,
            tags: vec!["mux".into(), "coordination".into(), action.into()],
        },
    );
}

fn quiet(reply: &CoordinationReply) -> bool {
    matches!(
        reply,
        CoordinationReply::Renewed { .. } | CoordinationReply::Released { count: 0 }
    )
}
