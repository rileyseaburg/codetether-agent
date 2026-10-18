//! Provider-error regression retained from the native smart switch module.
use super::super::*;

#[test]
fn model_key_normalizes() {
    assert_eq!(
        smart_switch_model_key("  Minimax-Credits/MiniMax-M2.5-Highspeed "),
        "minimax-credits/minimax-m2.5-highspeed".to_string()
    );
}
