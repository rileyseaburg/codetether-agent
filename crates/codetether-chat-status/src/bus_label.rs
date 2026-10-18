//! Connectivity and activity text/color decisions.
use crate::BusStatus;
use ratatui::style::Color;
/// Choose the bus status label and color.
///
/// # Arguments
/// * `status` - Connectivity and task-count snapshot.
///
/// # Returns
/// An owned label and its terminal color.
///
/// # Examples
/// ```
/// let _label = codetether_chat_status::bus_status_label_and_color(Default::default());
/// ```
pub fn bus_status_label_and_color(status: BusStatus) -> (String, Color) {
    let BusStatus {
        connected,
        agents,
        queued,
        processing,
        peer_ready,
    } = status;
    if connected || agents > 0 || queued > 0 {
        let state = if processing { "active" } else { "idle" };
        (
            format!("BUS {state} ({agents}ag/{queued}q)"),
            if processing {
                Color::Green
            } else {
                Color::Yellow
            },
        )
    } else if peer_ready {
        ("BUS peer ready".to_string(), Color::Cyan)
    } else {
        ("BUS offline".to_string(), Color::DarkGray)
    }
}
