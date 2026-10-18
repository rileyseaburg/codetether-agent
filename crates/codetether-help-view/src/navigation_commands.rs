//! Help navigation 1 content and presentation.
use crate::cmd_row::cmd_row;
use crate::heading::heading;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("Navigation"));
    lines.push(cmd_row("/chat", "", "Return to chat view"));
    lines.push(cmd_row("/help", "/h /?", "Open this help"));
    lines.push(cmd_row("/sessions", "/s", "Session picker"));
    lines.push(cmd_row("/model", "/m", "Model picker"));
    lines.push(cmd_row("/file", "", "Attach file contents to composer"));
    lines.push(cmd_row(
        "/image",
        "",
        "Attach image file (png, jpg, gif, webp, bmp, svg)",
    ));
    lines.push(cmd_row(
        "/autoapply",
        "/aa",
        "Toggle edit preview auto-apply",
    ));
    lines.push(cmd_row(
        "/network",
        "",
        "Toggle sandbox bash network access",
    ));
    lines.push(cmd_row(
        "/autocomplete",
        "",
        "Toggle slash-command Tab autocomplete",
    ));
}
