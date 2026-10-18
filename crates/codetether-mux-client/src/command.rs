//! Typed outcomes of the interactive mux command parser.
use codetether_mux_protocol::ClientRequest;

/// A mux control operation, terminal action or unchanged program command line.
/// # Examples
/// ```
/// match codetether_mux_client::parse("select 3") {
///     codetether_mux_client::ParsedCommand::Request(_) => {},
///     _ => panic!("expected a control request"),
/// }
/// ```
pub enum ParsedCommand {
    /// Structured request sent to the mux server.
    Request(ClientRequest),
    /// Program command line to start in the active window.
    Exec(String),
    /// Attach to the active window's existing program.
    Attach,
    /// Close the attached session; the server exits with its last session.
    Kill,
    /// Display command help.
    Help,
    /// Validation failure to display without sending a request.
    Invalid(String),
}
