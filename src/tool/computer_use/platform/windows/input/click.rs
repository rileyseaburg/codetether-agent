use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_click(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    let (x, y) = super::validate::coords(input)?;
    let script = format!(
        r#"Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Cursor]::Position=New-Object System.Drawing.Point({x},{y})
Add-Type -MemberDefinition '[DllImport("user32.dll")] public static extern void mouse_event(int f,int dx,int dy,int d,int e);' -Name U -Namespace W
[W.U]::mouse_event(6,0,0,0,0)
@{{clicked=$true;x={x};y={y}}}|ConvertTo-Json -Compress"#
    );
    Ok(super::super::response::success_result(
        super::super::ps::run(&script).await?,
    ))
}
