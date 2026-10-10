import Foundation

/// Owner-only reply body queued for Windows to type where the user last clicked.
struct ScreenReply: Encodable {
    let text: String
}

/// Acceptance means the reply is queued; Windows acks separately after typing.
struct ScreenReplyReceipt: Decodable {
    let reply_id: UUID
}

/// Session-only transports need not implement the separate reply capability.
@MainActor
protocol ScreenReplyNetworking {
    func send(_ reply: ScreenReply, session: UUID, token: String) async throws -> ScreenReplyReceipt
}
