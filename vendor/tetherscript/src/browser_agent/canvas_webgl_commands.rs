//! WebGL command attribute parsing.

use super::canvas_webgl_model::WebGlCommand;

pub(super) fn parse_command(raw: &str) -> Option<WebGlCommand> {
    if raw.is_empty() {
        return None;
    }
    let mut parts = raw.split('|');
    Some(WebGlCommand {
        operation: parts.next()?.into(),
        args: parts.map(str::to_string).collect(),
    })
}
