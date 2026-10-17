//! Crash reporting install panic hook.
use super::*;
pub(super) fn install_panic_hook(settings: CrashReporterSettings) {
    static PANIC_HOOK_ONCE: Once = Once::new();
    PANIC_HOOK_ONCE.call_once(|| {
        let default_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let persisted = persist_crash_report(&settings, panic_info);
            default_hook(panic_info);

            match persisted {
                Ok(path) if settings.enabled => {
                    eprintln!(
                        "codetether: crash report queued at '{}' and will be sent on next startup.",
                        path.display()
                    );
                }
                Ok(path) => {
                    eprintln!(
                        "codetether: crash report saved at '{}'. Crash reporting is disabled (opt-in).",
                        path.display()
                    );
                    eprintln!(
                        "codetether: enable with `codetether config --set telemetry.crash_reporting=true`"
                    );
                }
                Err(err) => {
                    eprintln!("codetether: failed to persist crash report: {err}");
                }
            }
        }));
    });
}
