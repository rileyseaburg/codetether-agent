//! Render a prepared objective for confirmation.
use super::Prompt;
pub(super) fn render(prompt: &Prompt<'_>) {
    let command_label = if prompt.easy_go_requested {
        "/go"
    } else {
        "/autochat"
    };
    println!("\n⚠️  {command_label} OKR Draft\n");
    println!("Task: {}", prompt.task_preview);
    println!("Agents: {} | Model: {}", prompt.agent_count, prompt.model);
    if let Some(note) = prompt.draft_note {
        println!("{}", note);
    }
    println!("\nObjective: {}", prompt.okr.title);
    println!("\nKey Results:");
    for kr in &prompt.okr.key_results {
        println!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit);
    }
    println!("\n");
}
