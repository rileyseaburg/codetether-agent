//! Application-specific session input enrichment.
use super::*;
impl codetether_native_agent::SessionServices for SessionServices {
    fn enrich(input: &Value, cwd: &Path, session: &Session) -> Value {
        crate::session::helper::runtime::enrich_tool_input_for_session(input, cwd, session)
    }
}
