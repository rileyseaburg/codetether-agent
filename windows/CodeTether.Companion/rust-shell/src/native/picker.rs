use super::{
    background,
    picker_items::{append, populate},
    state::{OFF, State},
};
use codetether_companion_desktop::validate_selection;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub(super) fn clear(state: &mut State) {
    background::cancel(state);
    state.selected = None;
    state.monitors.clear();
    if let Some(controls) = &state.controls {
        // SAFETY: the combo box is a live child and owns copies of its strings.
        unsafe {
            SendMessageW(controls.picker, CB_RESETCONTENT, 0, 0);
        }
        let _ = append(controls.picker, "Choose a monitor locally");
        unsafe {
            SendMessageW(controls.picker, CB_SETCURSEL, 0, 0);
        }
    }
}

/// Records the locally chosen monitor after rechecking its exact geometry.
/// Index 0 is the prompt. Selection authorizes this monitor for phone requests.
pub(super) fn choose(state: &mut State) {
    background::cancel(state);
    let Some(controls) = &state.controls else {
        return;
    };
    // SAFETY: the picker is a live child combo box on this thread.
    let index = unsafe { SendMessageW(controls.picker, CB_GETCURSEL, 0, 0) };
    state.selected = None;
    state.status = OFF;
    let Some(monitor) = usize::try_from(index)
        .ok()
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| state.monitors.get(i))
        .cloned()
    else {
        return;
    };
    if validate_selection(&monitor).is_ok() {
        state.selected = Some(monitor);
        state.status = "Monitor selected — pair with your iPhone for background requests.";
    } else {
        refresh(state);
    }
}

pub(super) fn refresh(state: &mut State) {
    clear(state);
    state.status = "Capture OFF — desktop unavailable. Refresh after returning to your desktop.";
    if let Ok(monitors) = codetether_companion_desktop::monitors() {
        if populate(state, &monitors).is_ok() {
            state.monitors = monitors;
            state.status = OFF;
        } else {
            clear(state);
        }
    }
}
