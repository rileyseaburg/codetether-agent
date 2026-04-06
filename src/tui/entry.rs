//! TUI entry point: terminal setup, teardown, and UUID parsing

use super::*;

fn parse_uuid_guarded(s: &str, context: &str) -> Option<Uuid> {
    match s.parse::<Uuid>() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(
                context,
                uuid_str = %s,
                error = %e,
                "Invalid UUID string - skipping operation to prevent NIL UUID corruption"
            );
            None

pub async fn run(project: Option<PathBuf>) -> Result<()> {
    // Change to project directory if specified
    if let Some(dir) = project {
        std::env::set_current_dir(&dir)?;
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let result = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    result
}

