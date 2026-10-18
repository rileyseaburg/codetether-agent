//! Native session binding for shared compact header encoding.
use crate::session::Session;
use anyhow::Result;
pub(super) fn encode(session: &Session) -> Result<String> {
    codetether_session_codec::encode_header(crate::session::codec::header(session))
}
pub(super) fn legacy(session: &Session) -> Result<String> {
    codetether_session_codec::legacy_header(crate::session::codec::header(session))
}
pub(super) fn decode(header: &str) -> Result<Session> {
    codetether_session_codec::decode_header(header)
}
