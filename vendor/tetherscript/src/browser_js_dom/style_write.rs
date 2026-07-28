use super::*;

pub(super) fn write(handle: &DomHandle, text: String) -> Result<(), String> {
    attr_update::change_style(handle, (!text.is_empty()).then_some(text))
}
