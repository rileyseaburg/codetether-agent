//! params for relay coordination.
use super::*;
pub(super) struct Params {
    pub(super) relay_id: Option<String>,
    pub(super) target_agent: Option<String>,
    pub(super) message: Option<String>,
    pub(super) context: Option<Value>,
    pub(super) okr_id: Option<String>,
    pub(super) task: Option<String>,
}
impl Params {
    pub(super) fn parse(params: &Value) -> Self {
        let relay_id = params
            .get("relay_id")
            .and_then(|v| v.as_str())
            .map(String::from);
        let target_agent = params
            .get("target_agent")
            .and_then(|v| v.as_str())
            .map(String::from);
        let message = params
            .get("message")
            .and_then(|v| v.as_str())
            .map(String::from);
        let context = params.get("context").cloned();
        let okr_id = params
            .get("okr_id")
            .and_then(|v| v.as_str())
            .map(String::from);
        let task = params
            .get("task")
            .and_then(|v| v.as_str())
            .map(String::from);

        Self {
            relay_id,
            target_agent,
            message,
            context,
            okr_id,
            task,
        }
    }
}
