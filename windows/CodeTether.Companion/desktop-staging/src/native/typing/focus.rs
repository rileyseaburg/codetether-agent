use crate::{Bounds, TypingError};
use windows::Win32::{System::Com::*, UI::Accessibility::*};
use windows_sys::Win32::Foundation::HWND;

pub(super) struct Target {
    pub(super) api: IUIAutomation2,
    pub(super) element: IUIAutomationElement,
    foreground: HWND,
    focus: HWND,
}
impl Target {
    pub(super) fn get(bounds: Bounds) -> Result<Self, TypingError> {
        // SAFETY: the calling worker holds its COM apartment through Target drop.
        let api: IUIAutomation2 =
            unsafe { CoCreateInstance(&CUIAutomation8, None, CLSCTX_INPROC_SERVER) }
                .map_err(|_| TypingError::Unavailable)?;
        unsafe {
            api.SetConnectionTimeout(500)
                .and_then(|_| api.SetTransactionTimeout(500))
        }
        .map_err(|_| TypingError::Unavailable)?;
        let (foreground, focus) = super::foreground::current()?;
        let element = unsafe { api.GetFocusedElement() }.map_err(|_| TypingError::Unavailable)?;
        let target = Self {
            api,
            element,
            foreground,
            focus,
        };
        target.check(bounds)?;
        Ok(target)
    }
    pub(super) fn check(&self, bounds: Bounds) -> Result<(), TypingError> {
        if super::foreground::current()? != (self.foreground, self.focus) {
            return Err(TypingError::TargetChanged);
        }
        super::editable::check(&self.element, bounds)?;
        // SAFETY: only reading live UIA properties, never setting values or focus.
        unsafe {
            let current = self
                .api
                .GetFocusedElement()
                .map_err(|_| TypingError::TargetChanged)?;
            if !self
                .api
                .CompareElements(&self.element, &current)
                .map_err(|_| TypingError::TargetChanged)?
                .as_bool()
            {
                return Err(TypingError::TargetChanged);
            }
        }
        if super::foreground::current()? != (self.foreground, self.focus) {
            return Err(TypingError::TargetChanged);
        }
        Ok(())
    }
}
