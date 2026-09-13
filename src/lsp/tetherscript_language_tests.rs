//! TetherScript resolves as a *language*, so `.tether` writes reach the
//! automatic pre-approval diagnostics pass (which looks servers up by language
//! id, not linter name). Without this a syntax error in a plugin was silently
//! unchecked at write time despite the linter registration.

use super::{TETHERSCRIPT_COMMAND, TETHERSCRIPT_LANGUAGE, TETHERSCRIPT_LINTER, tetherscript_args};
use crate::lsp::types::{
    detect_language_from_path, get_language_server_config, get_linter_server_config,
};

#[test]
fn plugin_extensions_map_to_the_tetherscript_language() {
    for path in [
        "plugin.tether",
        "legacy.kl",
        "examples/tetherscript/x.tether",
    ] {
        assert_eq!(
            detect_language_from_path(path),
            Some(TETHERSCRIPT_LANGUAGE),
            "{path}"
        );
    }
}

#[test]
fn language_server_config_starts_the_same_stdio_server_as_the_linter() {
    let config = get_language_server_config(TETHERSCRIPT_LANGUAGE)
        .expect("tetherscript must resolve as a language server for preflight");
    assert_eq!(config.command, TETHERSCRIPT_COMMAND);
    assert_eq!(config.args, tetherscript_args());
    let linter = get_linter_server_config(TETHERSCRIPT_LINTER).expect("linter config");
    assert_eq!(config.command, linter.command);
    assert_eq!(config.file_extensions, linter.file_extensions);
}

#[test]
fn other_languages_are_unaffected() {
    assert_eq!(detect_language_from_path("main.rs"), Some("rust"));
    assert!(get_language_server_config("rust").is_some());
    assert!(get_language_server_config("not-a-language").is_none());
}
