//! Non-chat inference-profile name exclusion.
use super::*;
pub(super) fn excluded(pname: &str) -> bool {
    let name_lower = pname.to_lowercase();
    let skip_tokens = [
        "image",
        "stable ",
        "upscale",
        "embed",
        "marengo",
        "outpaint",
        "inpaint",
        "erase",
        "recolor",
        "replace",
        "style ",
        "background",
        "sketch",
        "control",
        "transfer",
        "sonic",
        "pegasus",
        "rerank",
    ];
    skip_tokens.iter().any(|t| name_lower.contains(t))
}
