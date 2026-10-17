//! remember participant for relay coordination.
use super::*;
impl RelayAutoChatTool {
    pub(super) fn remember_participant(relay_id: &str, participant: &str) {
        let mut store = RELAY_STORE.write();
        if let Some(state) = store.get_mut(relay_id) {
            state.remember_participant(participant);
        }
    }
}
