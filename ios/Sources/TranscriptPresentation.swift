import Foundation

enum TranscriptPresentation {
    /// Hide only the server's generated continuation envelope from the chat UI.
    /// The full stored session remains unchanged and is still used by the agent.
    static func isRuntimeContinuation(_ text: String) -> Bool {
        text.hasPrefix("Continue working toward the active thread goal.")
            && text.contains("<objective>") && text.contains("Completion audit:")
    }
    static func displayText(_ text: String) -> String {
        let text = text.components(separatedBy: "\n\nUser attached image files:").first ?? text
        let visible = text.components(separatedBy: "\n\nRuntime scope ledger:").first ?? text
        return visible.isEmpty ? "Image attached" : visible
    }
}
