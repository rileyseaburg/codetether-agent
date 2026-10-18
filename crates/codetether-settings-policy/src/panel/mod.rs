//! Settings panel row order, bounded selection, and action dispatch.
//!
//! Use [select_previous] and [select_next] for cursor movement and [toggle]
//! to activate a row through the application's existing [Host] setters.
mod host;
mod selection;
use crate::{cycles::Cycle, persistence::Preference};
pub use host::Host;
pub use selection::{COUNT, select_next, select_previous};

/// Activate the selected settings row through its existing persistence path.
///
/// # Arguments
/// * `host` — Current preferences and native setting operations.
/// * `index` — Zero-based row index in the settings panel.
/// # Returns
/// Completes after the selected operation; out-of-range rows do nothing.
/// # Examples
/// ```no_run
/// # async fn activate(host: &mut impl codetether_settings_policy::panel::Host) {
/// codetether_settings_policy::panel::toggle(host, 0).await;
/// # }
/// ```
pub async fn toggle(host: &mut impl Host, index: usize) {
    let preference = match index {
        0 => Preference::AutoApply,
        1 => Preference::Network,
        2 => Preference::Autocomplete,
        3 => Preference::Worktree,
        4 => return host.cycle_access().await,
        5 => return host.cycle_provider(Cycle::BedrockTier).await,
        6 => return host.cycle_provider(Cycle::BedrockEffort).await,
        7 => return host.cycle_provider(Cycle::CodexEffort).await,
        8 => return host.cycle_provider(Cycle::OpenRouterEffort).await,
        _ => return,
    };
    let next = !host.current(preference);
    host.set_preference(preference, next).await;
}
