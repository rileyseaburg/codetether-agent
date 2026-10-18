use super::Host;
/// Insert normalized chat paste content, extracting images before text sidecars.
/// # Arguments
/// `host` supplies editing services; `normalized` has already normalized line endings.
/// # Returns
/// Completes after image handling, sidecar attachment or text insertion.
/// # Panics
/// May panic if allocating the next sidecar identifier overflows.
/// # Examples
/// ```no_run
/// # fn paste(host: &mut impl codetether_paste_buffer::input::Host) {
/// codetether_paste_buffer::input::paste_into_chat(host, "pasted text");
/// # }
/// ```
pub fn paste_into_chat(host: &mut impl Host, normalized: &str) {
    if host.try_attach_image(normalized) {
        return;
    }
    // A data URL pasted alongside caption text (or by a terminal that mixes
    // it into the buffer) is extracted here so it is not diverted into the
    // large-paste text sidecar and lost as an image.
    if let Some(found) = codetether_image::extract_image_data_url(normalized) {
        if host.try_attach_image(&found.data_url) {
            if !found.remainder.is_empty() {
                paste_into_chat(host, &found.remainder);
            }
            return;
        }
    }
    if crate::should_summarize(normalized) {
        super::summarise::attach_summarised_paste(host, normalized);
        return;
    }
    host.set_command_mode(if host.input().is_empty() && normalized.starts_with('/') {
        true
    } else if host.input().starts_with('/') {
        true
    } else {
        false
    });
    host.insert_text(normalized);
    let line_count = normalized.lines().count();
    host.set_status(if line_count > 1 {
        format!("Pasted {line_count} lines into input")
    } else {
        "Pasted into input".to_string()
    });
}
