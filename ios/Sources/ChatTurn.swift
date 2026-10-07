import Foundation

/// Stable ownership token for a single submitted message and all its async callbacks.
struct ChatTurn {
    let id = UUID()
    let message: ChatMessage
    let model: String
}