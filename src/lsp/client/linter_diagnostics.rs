//! linter diagnostics operations for language server clients.

use super::*;

impl LspManager {
    /// Collect diagnostics from all applicable linter servers for a file.
    /// Returns an empty vec if no linters match the file extension.
    pub async fn linter_diagnostics(&self, path: &Path) -> Vec<DiagnosticInfo> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Determine which linters apply based on file extension
        let linter_names: Vec<String> = if let Some(settings) = &self.lsp_settings {
            settings
                .linters
                .iter()
                .filter(|(_, entry)| {
                    entry.enabled
                        && (entry.file_extensions.iter().any(|e| e == ext)
                            || entry.file_extensions.is_empty())
                })
                .map(|(name, _)| name.clone())
                .collect()
        } else {
            // Auto-detect: try known linters whose extensions match
            let mut names = Vec::new();
            for candidate in crate::lsp::tetherscript::LINTER_CANDIDATES {
                if linter_extensions(candidate).contains(&ext) {
                    names.push((*candidate).to_string());
                }
            }
            names
        };

        let mut all_diagnostics = Vec::new();
        for name in &linter_names {
            match self.get_linter_client(name).await {
                Ok(Some(client)) => match client.diagnostics(path).await {
                    Ok(LspActionResult::Diagnostics { diagnostics }) => {
                        all_diagnostics.extend(diagnostics);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        debug!(linter = %name, error = %e, "Linter diagnostics failed");
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    debug!(linter = %name, error = %e, "Failed to get linter client");
                }
            }
        }
        all_diagnostics
    }
}
