//! Start a swarm after validating the current monitor state.

use super::spawn::spawn_swarm_run;
use super::{Host, View};

pub(super) fn start<H: Host>(app: &mut impl View, model: Option<String>, task: &str) -> bool {
    if app.swarm().active {
        app.set_status("A swarm is already running; cancel it before starting another.".into());
        return true;
    }

    let (tx, rx) = tokio::sync::mpsc::channel(256);
    app.swarm().attach_event_rx(rx);
    app.show_monitor();
    app.set_status(format!("Swarm running: {task}"));
    app.push_system(format!("Launching swarm for task: {task}"));
    app.scroll_to_bottom();

    app.swarm().control = Some(spawn_swarm_run::<H>(task.to_string(), model, tx));
    true
}
