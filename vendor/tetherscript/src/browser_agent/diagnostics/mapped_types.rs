//! Source-mapped production error report types.

/// Location in a generated JavaScript bundle.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::GeneratedSourceLocation;
///
/// let location = GeneratedSourceLocation {
///     script_url: "/assets/app.js".into(),
///     line: 1,
///     column: 23,
/// };
/// assert_eq!(location.script_url, "/assets/app.js");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedSourceLocation {
    /// Script resource URL that produced the error.
    pub script_url: String,
    /// One-based generated line number.
    pub line: usize,
    /// One-based generated column number.
    pub column: usize,
}

/// Original source location resolved through a source map.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::OriginalSourceLocation;
///
/// let location = OriginalSourceLocation {
///     source_url: "src/App.tsx".into(),
///     line: 10,
///     column: 5,
/// };
/// assert_eq!(location.line, 10);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalSourceLocation {
    /// Original source file path or URL from the source map.
    pub source_url: String,
    /// One-based original line number.
    pub line: usize,
    /// One-based original column number.
    pub column: usize,
}

/// One generated stack frame with optional source-map remapping.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{
///     GeneratedSourceLocation, SourceMappedStackFrame,
/// };
///
/// let frame = SourceMappedStackFrame {
///     function_name: Some("render".into()),
///     generated: GeneratedSourceLocation {
///         script_url: "/assets/app.js".into(),
///         line: 1,
///         column: 23,
///     },
///     original: None,
/// };
/// assert_eq!(frame.function_name.as_deref(), Some("render"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceMappedStackFrame {
    /// Function active for this frame, when the generated source exposes one.
    pub function_name: Option<String>,
    /// Generated bundle frame location.
    pub generated: GeneratedSourceLocation,
    /// Original source location when a registered map covers the frame.
    pub original: Option<OriginalSourceLocation>,
}

/// Page error with generated and optional original-source locations.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{
///     GeneratedSourceLocation, SourceMappedPageError,
/// };
///
/// let error = SourceMappedPageError {
///     action: "page.run_scripts".into(),
///     message: "ReferenceError: missing_call is not defined".into(),
///     generated: GeneratedSourceLocation {
///         script_url: "/assets/app.js".into(),
///         line: 1,
///         column: 23,
///     },
///     original: None,
///     stack: Vec::new(),
/// };
/// assert!(error.message.contains("missing_call"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceMappedPageError {
    /// Page action that produced the error.
    pub action: String,
    /// Runtime error message.
    pub message: String,
    /// Generated bundle location.
    pub generated: GeneratedSourceLocation,
    /// Original source location when a registered map covers the generated span.
    pub original: Option<OriginalSourceLocation>,
    /// Best-effort generated stack frames, each source-mapped when possible.
    pub stack: Vec<SourceMappedStackFrame>,
}
