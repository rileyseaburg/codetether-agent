//! Parser for compact `name=score` records.

/// A validated score record.
///
/// # Examples
///
/// ```
/// let record = record_parser::Record { name: "alice".into(), score: 42 };
/// assert_eq!(record.score, 42);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    /// Record name.
    pub name: String,
    /// Numeric score.
    pub score: u32,
}

/// Typed parsing failures.
///
/// Variants distinguish missing structure, empty names, and invalid scores.
///
/// # Examples
///
/// ```
/// use record_parser::ParseError;
/// let error = ParseError::MissingSeparator;
/// assert!(matches!(error, ParseError::MissingSeparator));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The `=` separator is absent.
    MissingSeparator,
    /// The name is empty after trimming.
    EmptyName,
    /// The score is not an unsigned integer.
    InvalidScore,
}

/// Parse one `name=score` record.
///
/// # Arguments
///
/// * `input` - Compact record text to validate and parse.
///
/// # Returns
///
/// A validated [`Record`] when parsing succeeds.
///
/// # Errors
///
/// Returns a [`ParseError`] variant for malformed input.
///
/// # Examples
///
/// ```
/// assert_eq!(record_parser::parse_record("alice=42").unwrap().score, 42);
/// ```
pub fn parse_record(input: &str) -> Result<Record, ParseError> {
    let (name, score) = input.split_once('=').unwrap();
    Ok(Record {
        name: name.to_string(),
        score: score.parse().unwrap(),
    })
}
