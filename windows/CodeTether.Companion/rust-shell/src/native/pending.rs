use super::{
    background, commands, context::Context, desktop, indicators, layout, pairing, state::State,
};
use anyhow::Result;
use windows_sys::Win32::Foundation::HWND;

/// Drain flags rather than discarding safety events during a nested callback.
pub(super) fn apply(context: &Context, hwnd: HWND, state: &mut State) -> Result<()> {
    loop {
        let invalidate = context.invalidate.replace(false);
        let relayout = context.relayout.replace(false);
        let command = context.command.take();
        if !invalidate && !relayout && command.is_none() {
            break;
        }
        if invalidate {
            desktop::invalidate(state);
        }
        if let Some(word) = command {
            commands::handle(hwnd, state, word);
        }
        if relayout {
            layout::update(hwnd, state)?;
        }
    }
    pairing::finish(state);
    desktop::check(state, context.session_blocked.get());
    background::tick(state);
    indicators::update(hwnd, state)?;
    Ok(())
}
