import Foundation
@testable import CodeTether

/// Isolated in-memory transport; never connects to a relay or requests real input.
@MainActor
final class ScreenTurnFixture: ScreenNetworking, ScreenQuestionNetworking, ScreenReplyNetworking {
    let receipt = ScreenSession(id: UUID(), code: "ABCDEF123456",
        pair_expires_at: "2099-01-01T00:00:00Z", expires_at: "2099-01-01T01:00:00Z", interval_seconds: 30)
    var requests: [UUID] = []
    var replies: [UUID] = []
    var streams = 0
    var failStreams = false
    var duringAsk: (() -> Void)?
    func create(_ body: ScreenCreateBody, token: String) async throws -> ScreenSession { receipt }
    func stop(_ id: UUID, token: String) async throws {}
    func events(_ id: UUID, token: String,
                receive: @escaping @MainActor (ScreenEvent) -> Void) async throws {
        streams += 1
        if failStreams { throw ScreenFailure.disconnected }
        receive(ScreenEvent(type: .snapshot, seq: 0, text: nil, status: "ready", captured_at: nil))
        try await Task.sleep(nanoseconds: 60_000_000_000)
    }
    func ask(_ question: ScreenQuestion, session: UUID, token: String) async throws -> ScreenQuestionReceipt {
        requests.append(session)
        duringAsk?()
        return ScreenQuestionReceipt(request_id: UUID())
    }
    func send(_ reply: ScreenReply, session: UUID, token: String) async throws -> ScreenReplyReceipt {
        replies.append(session)
        return ScreenReplyReceipt(reply_id: UUID())
    }
    func model() -> ScreenModel {
        let model = ScreenModel(client: self, token: { "fixture-owner" }, delay: { _ in })
        model.session = receipt; model.active = true
        emit(model, .snapshot, 0, "ready")
        return model
    }
    func emit(_ model: ScreenModel, _ type: ScreenEvent.Kind, _ seq: Int, _ status: String?) {
        model.receiveScreenEvent(ScreenEvent(type: type, seq: seq, text: nil,
                                             status: status, captured_at: nil))
    }
}
