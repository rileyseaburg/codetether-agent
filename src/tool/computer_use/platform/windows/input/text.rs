use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_type_text(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let text = input.text.as_deref().unwrap_or_default();
    let keys = super::super::ps::escape(&escape_send_keys_text(text));
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms;[System.Windows.Forms.SendKeys]::SendWait('{keys}');@{{typed=$true}}|ConvertTo-Json -Compress"
    );
    Ok(super::super::response::success_result(
        super::super::ps::run(&script).await?,
    ))
}

fn escape_send_keys_text(text: &str) -> String {
    text.chars().map(escape_send_keys_char).collect()
}

fn escape_send_keys_char(ch: char) -> String {
    match ch {
        '+' | '^' | '%' | '~' | '(' | ')' | '{' | '}' | '[' | ']' => format!("{{{ch}}}"),
        _ => ch.to_string(),
    }
}
