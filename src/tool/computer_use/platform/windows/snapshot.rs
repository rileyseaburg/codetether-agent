//! Windows snapshot handling for computer use.

pub fn handle_snapshot(
    _input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let script = r#"
Add-Type -AssemblyName System.Windows.Forms,System.Drawing
$b=[System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bmp=New-Object System.Drawing.Bitmap $b.Width,$b.Height
$g=[System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($b.Location,[System.Drawing.Point]::Empty,$b.Size)
$ms=New-Object System.IO.MemoryStream
$bmp.Save($ms,[System.Drawing.Imaging.ImageFormat]::Png)
[Convert]::ToBase64String($ms.ToArray())|ConvertTo-Json
"#;
    let png_base64 = super::ps::run(script)?;
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/png",
        "encoding": "base64",
        "image": png_base64
    })))
}
