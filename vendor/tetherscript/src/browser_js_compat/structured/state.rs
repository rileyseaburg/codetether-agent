use crate::js::JsValue;

use super::key::CloneKey;

#[derive(Default)]
pub(super) struct CloneState {
    active: Vec<CloneKey>,
    cloned: Vec<(CloneKey, JsValue)>,
}

impl CloneState {
    pub(super) fn cached(&self, key: CloneKey) -> Option<JsValue> {
        self.cloned
            .iter()
            .find(|(seen, _)| *seen == key)
            .map(|(_, value)| value.clone())
    }

    pub(super) fn enter(&mut self, key: CloneKey, name: &str) -> Result<(), String> {
        if self.active.contains(&key) {
            return Err(format!(
                "structuredClone: cyclic {} values are not supported",
                name
            ));
        }
        self.active.push(key);
        Ok(())
    }

    pub(super) fn leave(&mut self, key: CloneKey) {
        self.active.retain(|seen| *seen != key);
    }

    pub(super) fn remember(&mut self, key: CloneKey, value: JsValue) {
        self.cloned.push((key, value));
    }
}
