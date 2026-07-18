//! Immediate delivery to an active child prompt loop.

use super::Route;
use crate::session::helper::steering::{self, SteeringInput};

pub(crate) async fn steer(target: &str, owner: Option<&str>, message: &str) -> Route {
    let Some((agent_id, _)) = super::super::store::scope::resolve_for_parent(target, owner) else {
        return Route::NotFound;
    };
    if super::wait::steer_or_idle(&agent_id, message).await {
        Route::Steered
    } else {
        Route::Idle
    }
}

pub(super) fn push(agent_id: &str, message: &str) -> bool {
    let input = SteeringInput::new(message.into(), Vec::new());
    steering::push(agent_id, input)
}
