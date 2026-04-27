use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_press_key(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let key = input
        .key
        .as_deref()
        .or(input.text.as_deref())
        .unwrap_or("ENTER");
    let key = super::super::ps::escape(key);
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms;[System.Windows.Forms.SendKeys]::SendWait('{key}');@{{pressed='{key}'}}|ConvertTo-Json -Compress"
    );
    Ok(super::super::response::success_result(
        super::super::ps::run(&script).await?,
    ))
}
