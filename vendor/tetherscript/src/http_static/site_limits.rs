//! Static cache size limits.

const MAX_STATIC_FILES: usize = 8192;
const MAX_STATIC_BYTES: usize = 128 * 1024 * 1024;

/// Record one file and reject oversized static caches.
pub(crate) fn record_file(files: &mut usize, bytes: &mut usize, len: usize) -> Result<(), String> {
    *files += 1;
    *bytes += len;
    if *files > MAX_STATIC_FILES || *bytes > MAX_STATIC_BYTES {
        return Err("http_serve_static: static cache limit exceeded".into());
    }
    Ok(())
}
