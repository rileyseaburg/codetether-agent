import Foundation

/// Owner-only request body; the Windows device receives only the opaque request ID.
struct ScreenQuestion: Encodable {
    let question: String
}

/// Acceptance queues a fresh capture; analysis arrives separately on the event stream.
struct ScreenQuestionReceipt: Decodable {
    let request_id: UUID
}

/// Session-only transports need not implement the separate question capability.
@MainActor
protocol ScreenQuestionNetworking {
    func ask(_ question: ScreenQuestion, session: UUID, token: String) async throws -> ScreenQuestionReceipt
}
