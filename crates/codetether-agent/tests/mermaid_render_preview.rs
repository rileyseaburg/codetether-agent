//! Visual smoke test for mermaid TUI rendering.
//!
//! Prints each supported diagram layout so regressions in geometry are
//! visible in test output, and asserts the frame is well formed.

use codetether_agent::tui::chat::mermaid::render_block;

fn dump(name: &str, source: &str) -> String {
    let lines = render_block(source, 56).expect("diagram should parse");
    let text = lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("--- {name} ---\n{text}\n");
    text
}

#[test]
fn flowchart_td_renders_boxes_and_arrows() {
    let text = dump(
        "flowchart TD",
        "flowchart TD\n  A[Parse] --> B{Valid?}\n  B -->|yes| C(Render)\n  B -.-> D[Retry]",
    );
    assert!(text.starts_with("┌─ Mermaid ─"));
    assert!(text.trim_end().ends_with('─'));
    for label in ["Parse", "Valid?", "Render", "Retry"] {
        assert!(text.contains(label), "missing {label}");
    }
}

#[test]
fn flowchart_lr_renders_single_row_chain() {
    let text = dump(
        "flowchart LR",
        "flowchart LR\n  A[Read] --> B[Edit] --> C[Test]",
    );
    assert!(text.contains('▶'));
}

#[test]
fn sequence_diagram_renders_lifelines() {
    let text = dump(
        "sequenceDiagram",
        "sequenceDiagram\n  participant U as User\n  participant T as TUI\n  U->>T: prompt\n  T-->>U: diagram",
    );
    assert!(text.contains("User") && text.contains("TUI"));
    assert!(text.contains("prompt") && text.contains("diagram"));
}
