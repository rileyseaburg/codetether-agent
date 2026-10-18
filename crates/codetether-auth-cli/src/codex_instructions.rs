//! Browser and SSH callback instructions.
pub(super) fn show(authorization_url: &str) -> bool {
    println!("OpenAI Codex OAuth authentication");
    println!(
        "Sign in with your ChatGPT subscription account (Plus/Pro/Team/Enterprise) to use Codex models without API credits."
    );

    let is_ssh_session =
        std::env::var_os("SSH_CONNECTION").is_some() || std::env::var_os("SSH_TTY").is_some();
    if is_ssh_session {
        println!("Detected SSH session.");
        println!(
            "If your browser runs on your local machine, port-forward callback traffic first:"
        );
        println!("  ssh -L 1455:127.0.0.1:1455 <remote-host>");
        println!("Without forwarding, manual callback paste is still supported.");
    }

    println!("Open this URL: {}", authorization_url);
    println!(
        "After approving access, copy the browser callback URL and paste it below (it starts with http://localhost:1455/auth/callback)."
    );

    is_ssh_session
}
