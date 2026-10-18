//! Frontend bindings for periodic updates and visible-state tracking.
mod approval;
mod host;
mod retry;
mod snapshot;
mod worker;

/// Declare a frontend tick host, periodic update entry and audit refresh.
///
/// Subsystem callbacks keep frontend state ownership while the shared tick
/// runner orders updates and computes whether a new draw is required.
///
/// # Examples
/// See the frontend tick module for the concrete callback selections.
#[macro_export]
macro_rules! declare_frontend_tick {
    ($app:path, $mode:path;
     $question:path, $snapshot:path, $autochat:path, $voice:path,
     $swarm:path, $forage:path, $shell:path, $history:path, $symbols:path,
     $approvals:path; $audit:path $(,)?) => {
        $crate::__frontend_tick_host!(
            $app; $question, $snapshot, $autochat, $voice, $swarm, $forage,
            $shell, $history, $symbols, $approvals
        );
        /// Refresh audit data when the audit view is active.
        pub async fn refresh_audit(app: &mut $app) {
            use $mode as ViewMode;
            if app.state.view.mode == ViewMode::Audit {
                $audit(&mut app.state.audit).await;
            }
        }
        /// Apply one periodic tick to the frontend.
        pub async fn run(app: &mut $app) {
            $crate::run(&mut host::Native(app));
        }
    };
}
