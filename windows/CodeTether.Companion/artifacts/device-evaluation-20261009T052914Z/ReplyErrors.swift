import Foundation

/// Export only known, sanitized UI messages, never drafts, tokens or model answers.
enum ReplyErrors {
    static let messages = [
        "Analysis failed. Check the selected vision model and try again.",
        "Analysis interrupted or exceeded its limit. Request another capture.",
        "Windows did not provide a fresh screenshot within 60 seconds.",
        "Windows did not type the reply within 60 seconds.",
        "This screen session is unavailable or expired.",
        "Screen session is unavailable or expired.",
        "Sign in again using Server settings, then reconnect.",
        "This login cannot access the screen session.",
        "Pair Windows, or wait for the current capture, analysis, or queued reply to finish, then retry.",
        "Capture is paused until the iPhone stream reconnects.",
        "The capture or service limit was reached. Wait, or stop and create a new session.",
        "The screen service could not accept the request.",
        "The screen service sent an unexpected response.",
        "The screen stream disconnected.",
        "Enter a question or reply of 1–2,000 characters.",
        "Connection unavailable. Check Server settings and try again.",
        "Use one nonblank line of up to 2,000 characters; no Enter, tabs or control characters.",
        "Add your server bearer token in Settings.",
        "Token rejected (401). Update it in Settings.",
        "This token does not have permission (403).",
        "The server returned an unexpected response.",
        "Queued for Windows, not confirmed as typed. Check the Windows preview and target field.",
        "No automatic retry was sent. Check Windows before resending.",
        "Delivery was interrupted. The reply may already be queued."
    ]

    static func matching(_ labels: [String]) -> [String] {
        let fixed = messages.filter { message in labels.contains { $0.contains(message) } }
        let codes = labels.filter {
            $0.range(of: #"^The server returned HTTP [0-9]{3}\. Try again\.$"#,
                     options: .regularExpression) != nil
        }
        return Array(Set(fixed + codes)).sorted()
    }
}