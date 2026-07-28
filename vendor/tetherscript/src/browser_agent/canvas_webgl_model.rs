//! Typed WebGL metadata records.

/// One deterministic WebGL metadata command captured by the host.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::WebGlCommand;
///
/// let command = WebGlCommand {
///     operation: "clear".into(),
///     args: vec!["16384".into()],
/// };
/// assert_eq!(command.operation, "clear");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebGlCommand {
    /// WebGL operation name such as `viewport`, `clearColor`, or `clear`.
    pub operation: String,
    /// Stringified command arguments in call order.
    pub args: Vec<String>,
}

/// Deterministic non-rendering snapshot of one WebGL canvas context.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::WebGlContextSnapshot;
///
/// let snapshot = WebGlContextSnapshot {
///     version: 1,
///     width: 300,
///     height: 150,
///     viewport: [0, 0, 300, 150],
///     clear_color: [0.0, 0.0, 0.0, 0.0],
///     supported_extensions: Vec::new(),
///     commands: Vec::new(),
/// };
/// assert_eq!(snapshot.version, 1);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WebGlContextSnapshot {
    /// WebGL major version, `1` for `webgl`, `2` for `webgl2`.
    pub version: u8,
    /// Backing canvas width.
    pub width: u32,
    /// Backing canvas height.
    pub height: u32,
    /// Current viewport `[x, y, width, height]`.
    pub viewport: [i64; 4],
    /// Current clear color `[r, g, b, a]`.
    pub clear_color: [f64; 4],
    /// Deterministic supported extension names.
    pub supported_extensions: Vec<String>,
    /// Bounded WebGL command log.
    pub commands: Vec<WebGlCommand>,
}
