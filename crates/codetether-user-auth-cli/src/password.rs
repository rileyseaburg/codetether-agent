//! Terminal password entry with the original platform echo behavior.
use anyhow::Result;
use std::io::{self, Write};

/// Read a password using the original platform terminal behavior.
///
/// # Arguments
/// * `prompt` - Text displayed before reading standard input.
/// # Returns
/// The trimmed input; Unix temporarily disables echo.
/// # Errors
/// Returns prompt-flush or standard-input errors.
/// # Examples
/// ```no_run
/// let password = codetether_user_auth_cli::rpassword_prompt("Password: ")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn rpassword_prompt(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    // Disable echo on Unix
    #[cfg(unix)]
    {
        use std::io::BufRead;
        // Save terminal state
        let fd = 0; // stdin
        let orig = unsafe {
            let mut termios = std::mem::zeroed::<libc::termios>();
            libc::tcgetattr(fd, &mut termios);
            termios
        };

        // Disable echo
        unsafe {
            let mut termios = orig;
            termios.c_lflag &= !libc::ECHO;
            libc::tcsetattr(fd, libc::TCSANOW, &termios);
        }

        let mut password = String::new();
        let result = io::stdin().lock().read_line(&mut password);

        // Restore terminal state
        unsafe {
            libc::tcsetattr(fd, libc::TCSANOW, &orig);
        }
        println!(); // newline after password entry

        result?;
        Ok(password.trim().to_string())
    }

    #[cfg(not(unix))]
    {
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        Ok(password.trim().to_string())
    }
}
