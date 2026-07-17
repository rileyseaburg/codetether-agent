//! Publish assistant reasoning and visible text to the training bus.

use super::super::Runner;
use crate::a2a::types::Part;
use crate::bus::BusMessage;

/// Publish one reasoning segment for the current turn.
pub(super) fn thinking(runner: &Runner<'_>, step: usize, value: &str) {
    send(
        runner,
        format!("agent.{}.thinking", runner.session.agent),
        BusMessage::AgentThinking {
            agent_id: runner.session.agent.clone(),
            thinking: super::super::super::live_bus::compact_thinking(value),
            step,
        },
    );
}

/// Publish visible assistant text so a training turn has a completion.
pub(super) fn text(runner: &Runner<'_>, value: &str) {
    send(
        runner,
        format!("agent.{}.assistant", runner.session.agent),
        BusMessage::AgentMessage {
            from: runner.session.agent.clone(),
            to: "user".into(),
            parts: vec![Part::Text { text: value.into() }],
        },
    );
}

fn send(runner: &Runner<'_>, topic: String, message: BusMessage) {
    let Some(bus) = &runner.session.bus else {
        return;
    };
    bus.handle(&runner.session.agent).send_with_correlation(
        topic,
        message,
        Some(runner.progress.turn_id.clone()),
    );
}
