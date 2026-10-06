import Foundation

enum ChatModelSelection {
    /// Prefer the user's choice, then the subscription model exercised at installation.
    /// The server's default Bedrock credential is expired; do not silently retry on it.
    static func choose(models: [String], saved: String?, serverDefault: String?) -> String {
        for candidate in [saved, "openai-codex/gpt-5.5", serverDefault].compactMap({ $0 }) {
            if models.contains(candidate) { return candidate }
        }
        return models.first ?? ""
    }
}
