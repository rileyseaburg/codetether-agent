//! Pending-approval shortcut, editor and preview actions.
//!
//! [handle] dispatches approval shortcuts through a frontend [Host]. [open]
//! opens a patch-edit session, [copy_preview] exports clean preview text and
//! [scroll]/[wheel] use the existing bounded preview navigation policy.
mod copy;
mod decision;
mod edit;
mod host;
mod wheel;
pub use copy::copy_preview;
use crossterm::event::KeyEvent;
pub use decision::decide;
pub use edit::open;
pub use host::Host;
pub use wheel::handle as wheel;
/// Apply a preview navigation key using frontend state.
/// # Arguments
/// * `host` — Current preview state.
/// * `key` — Incoming navigation key.
/// # Returns
/// Whether the preview consumed the key.
pub fn scroll(host: &mut impl Host, key: KeyEvent) -> bool {
    crate::input::handle(host.preview(), key)
}
/// Route a pending-approval character shortcut.
/// # Arguments
/// * `host` — Frontend state and command bindings.
/// * `character` — Shortcut character.
/// * `cwd` — Workspace root for patch editing.
/// # Returns
/// Whether the action was consumed.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_view::interactive::Host) {
/// let _ = codetether_approval_view::interactive::handle(host, 'a', std::path::Path::new("."));
/// # }
/// ```
pub fn handle(host: &mut impl Host, character: char, cwd: &std::path::Path) -> bool {
    match character {
        'a' => decide(host, "/approve"),
        'd' => decide(host, "/deny"),
        'e' => open(host, cwd),
        'y' => copy_preview(host),
        _ => false,
    }
}
