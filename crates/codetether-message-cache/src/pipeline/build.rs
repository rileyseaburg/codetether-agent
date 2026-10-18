//! Full-cache, frozen-prefix and cold-render selection.
use super::{Host, focused, store::store_and_take};
use crate::DrawnLines;
/// Build chat lines using focused transcripts or the reusable frame cache.
/// # Arguments
/// Receives native render operations, cache width and inner content width.
/// # Returns
/// Owned rendered lines; restore them after drawing when they came from the cache.
/// # Examples
/// ```no_run
/// fn frame<H: codetether_message_cache::pipeline::Host>(host: &mut H) {
///     let lines = codetether_message_cache::pipeline::build(host, 80, 76);
///     lines.restore(host.cache(), 80);
/// }
/// ```
pub fn build(host: &mut impl Host, max_width: usize, content_width: usize) -> DrawnLines {
    if let Some(lines) = focused::build(host, content_width) {
        return lines;
    }
    let inputs = host.cache_inputs();
    if let Some(lines) = host.cache().take_cached_if_valid(max_width, inputs) {
        return DrawnLines::from_cache(lines);
    }
    let separator_width = content_width.saturating_sub(2).min(60);
    let panel_width = content_width.saturating_sub(4);
    let inputs = host.cache_inputs();
    if let Some(mut prefix) = host.cache().take_frozen_prefix(max_width, inputs) {
        let frozen_len = prefix.len();
        host.push_streaming(&mut prefix, separator_width);
        return store_and_take(host, prefix, max_width, frozen_len);
    }
    let mut built = host.build_uncached(separator_width, panel_width);
    let frozen_len = built.len();
    host.push_streaming(&mut built, separator_width);
    store_and_take(host, built, max_width, frozen_len)
}
