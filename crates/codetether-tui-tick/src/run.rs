use super::Host;
/// Drain periodic UI updates in order and preserve the redraw short-circuit.
/// # Arguments
/// Host supplies all subsystem drains, snapshot capture and redraw state.
/// # Returns
/// Completes after every drain; captures a second snapshot only if no drain
/// explicitly reported a change.
/// # Examples
/// ```ignore
/// run(&mut native_tui);
/// ```
pub fn run<H: Host>(host: &mut H) {
    host.question_wait();
    let before = host.snapshot();
    host.autochat();
    host.voice();
    let ralph_changed = host.ralph();
    let swarm_changed = host.swarm();
    let forage_changed = host.forage();
    let shell_changed = host.shell();
    let history_changed = host.history();
    let symbols_changed = host.symbols();
    let approval_changed = host.approvals();
    let changed = ralph_changed
        || swarm_changed
        || forage_changed
        || shell_changed
        || history_changed
        || symbols_changed
        || approval_changed
        || before != host.snapshot();
    host.mark_redraw(changed);
}
