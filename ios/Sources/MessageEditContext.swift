import Foundation

/// Resolve a visible user prompt to its raw persisted position, preserving attachments.
struct MessageEditContext {
    let index: Int
    let original: String
    var attachmentSuffix: String {
        let markers = ["\n\nUser attached image files:", "\n\nUser attached PDF documents:"]
        let starts = markers.compactMap { original.range(of: $0)?.lowerBound }
        guard let start = starts.min() else { return "" }
        return String(original[start...])
    }

    static func find(_ message: ChatMessage, in visible: [ChatMessage], snapshot: AgentSession) throws -> Self {
        guard let offset = visible.firstIndex(where: { $0.id == message.id }), message.role == "user" else {
            throw ClientError.agent("This message is no longer available to edit.")
        }
        let ordinal = visible[..<offset].filter { $0.role == "user" }.count
        let candidates = (snapshot.messages ?? []).enumerated().compactMap { index, value -> Self? in
            guard value.role == "user" else { return nil }
            let raw = value.content.compactMap { $0.type == "text" ? $0.text : nil }.joined(separator: "\n")
            guard !raw.isEmpty, !TranscriptPresentation.isRuntimeContinuation(raw) else { return nil }
            return Self(index: index, original: raw)
        }
        guard candidates.indices.contains(ordinal) else {
            throw ClientError.agent("The saved conversation changed. Reopen it before editing.")
        }
        let target = candidates[ordinal]
        let expected = message.content.isEmpty ? "Describe the attached image." : message.content
        guard TranscriptPresentation.displayText(target.original) == expected else {
            throw ClientError.agent("The saved message changed. Reopen the conversation before editing.")
        }
        return target
    }
}