//! Short CLI usage text.

pub(crate) fn print_usage() {
    eprintln!("Usage: tetherscript <command> [options]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  run <file>           Run a TetherScript program");
    eprintln!("  inspect <file>       Inspect source (tokens, AST, bytecode)");
    eprintln!("  render <html>        Render HTML/CSS display list");
    eprintln!("  raster <html> <ppm>  Render HTML/CSS to a PPM image");
    eprintln!("  js <file.js>         Run JavaScript with the built-in engine");
    eprintln!("  git                  Show first-class git workspace status");
    eprintln!("  repl                 Interactive REPL");
    eprintln!("  lsp                  Start LSP server over stdio");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help           Show help");
    eprintln!("  -V, --version        Show version");
    eprintln!();
    eprintln!("Run 'tetherscript <command> --help' for more on a command.");
    eprintln!();
    eprintln!("Legacy: tetherscript <file.tether> also works (same as 'run').");
}
