//! Release of paused approval requests after switching to full access.

use crate::tui::app::state::{App, approval_queue};

pub(crate) fn all(app: &mut App) -> Result<usize, String> {
    let mut released = 0;
    loop {
        let before = target();
        let Some(before) = before else {
            break;
        };
        if !super::run(app, "/approve") {
            return Err("approval command unavailable".into());
        }
        if target().as_ref() == Some(&before) {
            return Err(app.state.status.clone());
        }
        released += 1;
    }
    Ok(released)
}

fn target() -> Option<String> {
    approval_queue::active_id().or_else(crate::approval::live::latest_id)
}

#[cfg(test)]
#[path = "approval_command_release_tests.rs"]
mod tests;
