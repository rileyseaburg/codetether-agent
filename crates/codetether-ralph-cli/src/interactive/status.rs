//! Interactive Ralph progress status formatting.
use super::View;
use codetether_ralph_core::events::RalphStoryStatus;
pub(super) fn show<V: View>(view: &mut V) {
    let stories = view.stories();
    if stories.is_empty() {
        view.set_status("No Ralph run attached".to_string());
    } else {
        let passed = stories
            .iter()
            .filter(|s| matches!(s.status, RalphStoryStatus::Passed))
            .count();
        view.set_status(format!(
            "Ralph: {}/{} stories passed (iteration {}/{})",
            passed,
            stories.len(),
            view.current_iteration(),
            view.max_iterations(),
        ));
    }
}
