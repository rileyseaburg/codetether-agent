//! Forage selected reporting.
use super::*;
impl RunEvents<'_> {
    pub(in crate::forage) fn selected(
        &self,
        cycle_task_id: &str,
        cycle: usize,
        top: usize,
        selected: &[ForageOpportunity],
    ) {
        let _ = self.bus_handle.send(
            format!("forage.{cycle_task_id}.summary"),
            BusMessage::SharedResult {
                key: format!("forage/cycle/{cycle}/summary"),
                value: json!({
                    "cycle": cycle,
                    "selected": selected.len(),
                    "top": top,
                }),
                tags: vec![
                    "forage".to_string(),
                    "okr".to_string(),
                    "summary".to_string(),
                ],
            },
        );
        let _ = self.bus_handle.send_to_agent(
            "user",
            vec![Part::Text {
                text: format!(
                    "Forage cycle {cycle}: selected {} opportunities from OKR governance queue.",
                    selected.len()
                ),
            }],
        );
    }
}
