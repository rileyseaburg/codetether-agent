//! Overlay scroll priority for symbol, session, model and slash lists.
use crate::Action;
use codetether_tui_models::ViewMode;
pub(super) fn scroll_overlay_up(host: &mut impl crate::Host, amount: usize) -> bool {
    let context = host.context();
    if context.symbols_active {
        for _ in 0..amount {
            host.apply(Action::SymbolPrevious);
        }
        return true;
    }
    if context.view_mode == ViewMode::Sessions {
        for _ in 0..amount {
            host.apply(Action::SessionPrevious);
        }
        return true;
    }
    if context.view_mode == ViewMode::Model {
        for _ in 0..amount {
            host.apply(Action::ModelPrevious);
        }
        return true;
    }
    if context.slash_navigable {
        for _ in 0..amount {
            host.apply(Action::SlashPrevious);
        }
        return true;
    }
    false
}

/// Try to scroll an overlay downward.
///
/// Mirror of [`scroll_overlay_up`] for the down direction.
///
/// # Examples
///
/// ```ignore
/// if scroll_overlay_down(&mut app) { return; }
/// ```
pub(super) fn scroll_overlay_down(host: &mut impl crate::Host, amount: usize) -> bool {
    let context = host.context();
    if context.symbols_active {
        for _ in 0..amount {
            host.apply(Action::SymbolNext);
        }
        return true;
    }
    if context.view_mode == ViewMode::Sessions {
        for _ in 0..amount {
            host.apply(Action::SessionNext);
        }
        return true;
    }
    if context.view_mode == ViewMode::Model {
        for _ in 0..amount {
            host.apply(Action::ModelNext);
        }
        return true;
    }
    if context.slash_navigable {
        for _ in 0..amount {
            host.apply(Action::SlashNext);
        }
        return true;
    }
    false
}
