//! Arrow function rewriting for module resources.

use super::{script_arrow_body, script_arrow_params};

pub(crate) fn rewrite(source: &str) -> String {
    let mut out = source.to_string();
    while let Some(arrow) = out.find("=>") {
        let Some(params) = script_arrow_params::find(&out, arrow) else {
            break;
        };
        let start = skip_ws(&out, arrow + 2);
        let Some(body) = script_arrow_body::find(&out, start) else {
            break;
        };
        out.replace_range(
            params.start..body.end,
            &replacement(&params.text, &body.text, body.block),
        );
    }
    out
}

fn replacement(params: &str, body: &str, block: bool) -> String {
    if block {
        format!("function({}){}", params, body)
    } else {
        format!("function({}){{ return {}; }}", params, body.trim())
    }
}

fn skip_ws(source: &str, mut index: usize) -> usize {
    while source
        .as_bytes()
        .get(index)
        .is_some_and(u8::is_ascii_whitespace)
    {
        index += 1;
    }
    index
}
