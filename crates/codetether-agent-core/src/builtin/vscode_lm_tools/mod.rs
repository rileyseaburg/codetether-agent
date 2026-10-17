//! VS Code language model tool discovery for project prompts.

mod discover;
mod render;
mod summary;
mod text;
mod tool_line;

#[cfg(test)]
mod tests;

pub(super) fn render_section(cwd: &std::path::Path) -> String {
    render::render_section(cwd)
}
