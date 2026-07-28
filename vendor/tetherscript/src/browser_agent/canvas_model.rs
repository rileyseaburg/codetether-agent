//! Typed canvas surface records.

/// One deterministic Canvas 2D command captured by the host.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::CanvasCommand;
///
/// let command = CanvasCommand {
///     operation: "fillRect".into(),
///     args: vec![1, 2, 3, 4],
///     style: Some("#f00".into()),
/// };
/// assert_eq!(command.operation, "fillRect");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanvasCommand {
    /// Canvas operation name such as `fillRect` or `clearRect`.
    pub operation: String,
    /// Integer command arguments in call order.
    pub args: Vec<i64>,
    /// Optional style associated with paint commands.
    pub style: Option<String>,
}

/// Deterministic snapshot of one `<canvas>` element.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::CanvasSurface;
///
/// let surface = CanvasSurface {
///     width: 300,
///     height: 150,
///     commands: Vec::new(),
///     checksum: Some(0),
/// };
/// assert_eq!((surface.width, surface.height), (300, 150));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanvasSurface {
    /// Canvas bitmap width in CSS pixels.
    pub width: u32,
    /// Canvas bitmap height in CSS pixels.
    pub height: u32,
    /// Captured drawing command log.
    pub commands: Vec<CanvasCommand>,
    /// Deterministic checksum of the current native raster buffer, when present.
    pub checksum: Option<u64>,
}
