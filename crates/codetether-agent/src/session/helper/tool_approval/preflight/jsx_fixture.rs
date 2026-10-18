//! Dependency-free JSX project for real TypeScript language-server checks.

use std::path::{Path, PathBuf};

pub(super) const ORIGINAL: &str =
    "export const LayerPicker = () => <section><span>Layers</span></section>;\n";
pub(super) const PROPOSED: &str =
    "export const MetadataInspector = () => <section><span>Metadata</span></section>;\n";

pub(super) fn available() -> bool {
    std::process::Command::new("typescript-language-server")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub(super) fn project(root: &Path, extension: &str) -> PathBuf {
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("tsconfig.json"),
        r#"{
        "compilerOptions": {"strict": true, "jsx": "preserve", "allowJs": true,
            "checkJs": true, "noEmit": true},
        "include": ["src/**/*"]
    }"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/jsx.d.ts"),
        r#"
        declare namespace JSX {
            interface Element {}
            interface IntrinsicElements {
                section: { children?: unknown };
                span: { children?: unknown };
            }
        }
    "#,
    )
    .unwrap();
    let path = root.join(format!("src/MetadataInspector.{extension}"));
    std::fs::write(&path, ORIGINAL).unwrap();
    path
}
