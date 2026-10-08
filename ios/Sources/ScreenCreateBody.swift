import Foundation

/// Frozen session configuration sent once; later chat model changes do not affect it.
struct ScreenCreateBody: Encodable {
    let model: String
    let prompt: String
    let interval_seconds: Int

    var valid: Bool {
        let parts = model.split(separator: "/", maxSplits: 1, omittingEmptySubsequences: false)
        return parts.count == 2 && parts.allSatisfy { !$0.isEmpty }
            && !model.contains(where: { $0.isWhitespace || $0.isNewline })
            && prompt.count <= 2000 && (15...300).contains(interval_seconds)
            && !prompt.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }
}

/// DELETE must explicitly acknowledge revocation before the owner handle is dropped.
struct ScreenStopReceipt: Decodable {
    let stopped: Bool
}