//! Formatter width and construction.

impl crate::MessageFormatter {
    /// Create a formatter for a terminal width.
    /// # Arguments
    /// * `max_width` - Available terminal columns.
    /// # Returns
    /// A formatter retaining the supplied width.
    /// # Examples
    /// ```
    /// assert_eq!(codetether_message_formatter::MessageFormatter::new(80).max_width(), 80);
    /// ```
    pub fn new(max_width: usize) -> Self {
        Self { max_width }
    }

    /// Configured maximum wrap width for this formatter.
    /// Read the configured terminal width.
    /// # Arguments
    /// None.
    /// # Returns
    /// The configured width in columns.
    /// # Examples
    /// ```
    /// assert_eq!(codetether_message_formatter::MessageFormatter::new(40).max_width(), 40);
    /// ```
    pub fn max_width(&self) -> usize {
        self.max_width
    }
}
