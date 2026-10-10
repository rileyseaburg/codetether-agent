use crate::{Bounds, TypingError};
use windows::Win32::UI::Accessibility::*;

pub(super) fn check(element: &IUIAutomationElement, bounds: Bounds) -> Result<(), TypingError> {
    // SAFETY: live UIA interface in this thread's COM apartment; queries only.
    let allowed = unsafe { inspect(element, bounds) }.map_err(|_| TypingError::Unavailable)?;
    if allowed {
        Ok(())
    } else {
        Err(TypingError::Unavailable)
    }
}
unsafe fn inspect(element: &IUIAutomationElement, bounds: Bounds) -> windows::core::Result<bool> {
    // SAFETY: caller guarantees apartment/interface lifetimes.
    unsafe {
        let kind = element.CurrentControlType()?;
        if (kind != UIA_EditControlTypeId && kind != UIA_DocumentControlTypeId)
            || element.CurrentIsPassword()?.as_bool()
            || element.CurrentIsOffscreen()?.as_bool()
            || !element.CurrentIsEnabled()?.as_bool()
            || !element.CurrentHasKeyboardFocus()?.as_bool()
        {
            return Ok(false);
        }
        let rect = element.CurrentBoundingRectangle()?;
        if rect.right <= rect.left
            || rect.bottom <= rect.top
            || !bounds.contains(rect.left, rect.top)
            || !bounds.contains(rect.right - 1, rect.bottom - 1)
        {
            return Ok(false);
        }
        if let Ok(value) =
            element.GetCurrentPatternAs::<IUIAutomationValuePattern>(UIA_ValuePatternId)
        {
            return Ok(!value.CurrentIsReadOnly()?.as_bool());
        }
        // TextEdit (unlike Text) positively identifies an editable text provider.
        Ok(element
            .GetCurrentPatternAs::<IUIAutomationTextEditPattern>(UIA_TextEditPatternId)
            .is_ok())
    }
}
