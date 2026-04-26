//! Windows input handling for computer use.

use crate::tool::computer_use::input::ComputerUseInput;

pub fn handle_click(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = coords(input)?;
    let script = format!(
        r#"Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Cursor]::Position=New-Object System.Drawing.Point({x},{y})
Add-Type -MemberDefinition '[DllImport("user32.dll")] public static extern void mouse_event(int f,int dx,int dy,int d,int e);' -Name U -Namespace W
[W.U]::mouse_event(6,0,0,0,0)
@{{clicked=$true;x={x};y={y}}}|ConvertTo-Json"#
    );
    Ok(super::response::success_result(super::ps::run(&script)?))
}

pub fn handle_type_text(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let text = super::ps::escape(input.text.as_deref().unwrap_or_default());
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms;[System.Windows.Forms.SendKeys]::SendWait('{text}');@{{typed=$true}}|ConvertTo-Json"
    );
    Ok(super::response::success_result(super::ps::run(&script)?))
}

pub fn handle_press_key(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let key = super::ps::escape(input.text.as_deref().unwrap_or("ENTER"));
    let script = format!(
        "Add-Type -AssemblyName System.Windows.Forms;[System.Windows.Forms.SendKeys]::SendWait('{{{key}}}');@{{pressed='{key}'}}|ConvertTo-Json"
    );
    Ok(super::response::success_result(super::ps::run(&script)?))
}

pub fn handle_scroll(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let amount = input.y.unwrap_or(-120.0) as i32;
    let script = format!(
        "Add-Type -MemberDefinition '[DllImport(\"user32.dll\")] public static extern void mouse_event(int f,int dx,int dy,int d,int e);' -Name U -Namespace W;[W.U]::mouse_event(2048,0,0,{amount},0);@{{scrolled={amount}}}|ConvertTo-Json"
    );
    Ok(super::response::success_result(super::ps::run(&script)?))
}

pub fn handle_stop() -> anyhow::Result<crate::tool::ToolResult> {
    Ok(super::response::success_result(
        serde_json::json!({"stopped": true}),
    ))
}

fn coords(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    let x = input.x.ok_or_else(|| anyhow::anyhow!("x is required"))? as i32;
    let y = input.y.ok_or_else(|| anyhow::anyhow!("y is required"))? as i32;
    Ok((x, y))
}
