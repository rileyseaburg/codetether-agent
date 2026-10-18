use super::State;

/// Handle a paste that is entirely one image data URL.
/// # Arguments
/// `state` borrows the composer fields; `text` is the pasted data URL.
///
/// # Returns
///
/// `true` when the paste was consumed — either attached as an image, or
/// recognised as an image that cannot be attached (for example a screenshot
/// over the size cap) and reported to the user. Consuming a rejected image
/// keeps megabytes of base64 out of the large-paste text sidecar, where the
/// failure would otherwise be silent.
/// # Examples
/// ```
/// let (mut images, mut input, mut cursor, mut status) = (Vec::new(), String::new(), 0, String::new());
/// let mut state = codetether_image::paste::State {
///     images: &mut images, input: &mut input, cursor: &mut cursor, status: &mut status,
/// };
/// assert!(!codetether_image::paste::try_attach_data_url(&mut state, "plain text"));
/// ```
pub fn try_attach_data_url(state: &mut State<'_>, text: &str) -> bool {
    let Some(image) = crate::attachment_from_data_url(text) else {
        let Some(reason) = crate::reject_reason(text) else {
            return false;
        };
        // A terminal may deliver the remaining base64 in later paste events.
        // Preserve incomplete content for sidecar reassembly at submission.
        if reason == crate::RejectReason::MalformedBase64 {
            return false;
        }
        *state.status = reason.message();
        return true;
    };
    state.images.push(image);
    let count = state.images.len();
    *state.status = if count == 1 {
        "Attached pasted image. Press Enter to send, or add text first.".to_string()
    } else {
        format!("Attached {count} pasted images. Press Enter to send, or add text first.")
    };
    true
}
