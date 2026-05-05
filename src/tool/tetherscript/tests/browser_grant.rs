//! Test browser capability grant fields in tetherscript_plugin input.

#[cfg(test)]
#[cfg(feature = "tetherscript")]
mod tests {
    use crate::tool::tetherscript::input::TetherScriptPluginInput;

    #[test]
    fn parses_browser_grant_fields() {
        let input: TetherScriptPluginInput = serde_json::from_str(
            r#"{
            "hook": "main",
            "grant_browser": "http://127.0.0.1:41707/browser",
            "browser_origin": ["http://localhost:5173"],
            "browser_scope": ["browser.navigate", "browser.interact"]
        }"#,
        )
        .unwrap();
        assert_eq!(input.hook, "main");
        assert!(input.wants_browser());
        assert_eq!(
            input.grant_browser.as_deref(),
            Some("http://127.0.0.1:41707/browser")
        );
        assert_eq!(input.browser_origin, ["http://localhost:5173"]);
        assert_eq!(
            input.browser_scope,
            ["browser.navigate", "browser.interact"]
        );
    }

    #[test]
    fn no_browser_grant_when_absent() {
        let input: TetherScriptPluginInput = serde_json::from_str(r#"{"hook": "main"}"#).unwrap();
        assert!(!input.wants_browser());
        assert!(input.grant_browser.is_none());
        assert!(input.browser_origin.is_empty());
        assert!(input.browser_scope.is_empty());
    }
}
