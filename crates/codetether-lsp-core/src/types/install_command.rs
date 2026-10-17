//! LSP install command definitions and helpers.

/// Returns the install command for a language server binary, if known.
pub(super) fn install_command_for(command: &str) -> Option<&'static [&'static str]> {
    match command {
        "rust-analyzer" => Some(&["rustup", "component", "add", "rust-analyzer"]),
        "typescript-language-server" => Some(&[
            "npm",
            "install",
            "-g",
            "typescript-language-server",
            "typescript",
        ]),
        "pylsp" => Some(&["pip", "install", "--user", "python-lsp-server"]),
        "gopls" => Some(&["go", "install", "golang.org/x/tools/gopls@latest"]),
        "tetherscript" => Some(&["cargo", "install", "tetherscript"]),
        "clangd" => None, // system package manager varies
        _ => None,
    }
}
