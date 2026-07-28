//! File payload values used by file-input upload actions.

/// Deterministic file metadata for [`BrowserPage::set_input_files`].
///
/// The payload stores compact in-memory bytes so tests can assert file sizes
/// without touching the host filesystem.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::FilePayload;
///
/// let file = FilePayload::new("note.txt", "text/plain", b"hello".to_vec());
/// assert_eq!(file.name, "note.txt");
/// assert_eq!(file.mime_type, "text/plain");
/// assert_eq!(file.byte_len(), 5);
/// ```
///
/// [`BrowserPage::set_input_files`]: crate::browser_agent::BrowserPage::set_input_files
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePayload {
    /// Browser-visible file name.
    pub name: String,
    /// Browser-visible MIME type.
    pub mime_type: String,
    /// In-memory bytes used to expose a deterministic file size.
    pub bytes: Vec<u8>,
}

impl FilePayload {
    /// Create a file payload from a name, MIME type, and bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::FilePayload;
    ///
    /// let file = FilePayload::new("avatar.png", "image/png", vec![1, 2, 3]);
    /// assert_eq!(file.byte_len(), 3);
    /// ```
    pub fn new(
        name: impl Into<String>,
        mime_type: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            name: name.into(),
            mime_type: mime_type.into(),
            bytes: bytes.into(),
        }
    }

    /// Return the byte length exposed to the browser-side metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::FilePayload;
    ///
    /// let file = FilePayload::new("data.bin", "application/octet-stream", vec![0, 1]);
    /// assert_eq!(file.byte_len(), 2);
    /// ```
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }
}
