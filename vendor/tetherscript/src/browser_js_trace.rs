//! Optional browser JavaScript phase tracing.

pub(super) fn mark(phase: &str) {
    if std::env::var_os("TETHERSCRIPT_BROWSER_JS_TRACE").is_some() {
        eprintln!("browser_js: {phase}");
    }
}
