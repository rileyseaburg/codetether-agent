//! Registration tests: TetherScript is a linter, not a language server.

use super::tetherscript::{
    LINTER_CANDIDATES, TETHERSCRIPT_COMMAND, TETHERSCRIPT_EXTENSIONS, TETHERSCRIPT_LINTER,
    tetherscript_args,
};
use super::types::{get_linter_server_config, linter_extensions};

#[test]
fn tetherscript_linter_config_starts_the_stdio_language_server() {
    let config = get_linter_server_config(TETHERSCRIPT_LINTER)
        .expect("tetherscript must be a known builtin linter");
    assert_eq!(config.command, TETHERSCRIPT_COMMAND);
    assert_eq!(config.args, tetherscript_args());
}

#[test]
fn tetherscript_linter_claims_plugin_extensions() {
    let extensions = linter_extensions(TETHERSCRIPT_LINTER);
    assert!(extensions.contains(&"tether"), "got {extensions:?}");
    assert!(
        extensions.contains(&"kl"),
        "legacy .kl plugins must still lint: {extensions:?}"
    );
    let config = get_linter_server_config(TETHERSCRIPT_LINTER).expect("builtin config");
    for ext in TETHERSCRIPT_EXTENSIONS {
        assert!(
            config.file_extensions.contains(&(*ext).to_string()),
            "config missing {ext}"
        );
    }
}

#[test]
fn tetherscript_is_probed_during_linter_auto_detect() {
    assert!(LINTER_CANDIDATES.contains(&TETHERSCRIPT_LINTER));
    for existing in ["eslint", "biome", "ruff", "stylelint"] {
        assert!(
            LINTER_CANDIDATES.contains(&existing),
            "auto-detect regressed for {existing}"
        );
    }
}

#[test]
fn tetherscript_is_not_registered_as_a_general_language_server() {
    // `tetherscript lsp` implements diagnostics only, so it must not be offered
    // for go-to-definition or hover requests via the language-server path.
    assert!(super::types::get_language_server_config(TETHERSCRIPT_LINTER).is_none());
    assert!(super::types::detect_language_from_path("plugin.tether").is_none());
}
