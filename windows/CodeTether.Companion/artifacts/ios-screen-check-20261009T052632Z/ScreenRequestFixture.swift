import Foundation
@testable import CodeTether

/// In-memory owner transport. No device, credentials, or live relay is used.
@MainActor
final class ScreenRequestFixture: ScreenNetworking, ScreenQuestionNetworking, ScreenReplyNetworking {
    let receipt = ScreenSession(id: UUID(), code: "ABCDEF123456",
        pair_expires_at: "2099-01-01T00:00:00Z", expires_at: "2099-01-01T01:00:00Z", interval_seconds: 30)
    var questions: [String] = []
    var replies: [String] = []
    var failure: ScreenFailure?
    var hold = false
    var pending: CheckedContinuation<Void, Never>?
    func create(_ body: ScreenCreateBody, token: String) async throws -> ScreenSession { receipt }
    func stop(_ id: UUID, token: String) async throws {}
    func events(_ id: UUID, token: String,
                receive: @escaping @MainActor (ScreenEvent) -> Void) async throws {
        try await Task.sleep(nanoseconds: 60_000_000_000)
    }
    func ask(_ question: ScreenQuestion, session: UUID, token: String) async throws -> ScreenQuestionReceipt {
        questions.append(question.question)
        if hold { await withCheckedContinuation { pending = $0 } }
        if let failure { throw failure }
        return ScreenQuestionReceipt(request_id: UUID())
    }
    func send(_ reply: ScreenReply, session: UUID, token: String) async throws -> ScreenReplyReceipt {
        replies.append(reply.text)
        if let failure { throw failure }
        return ScreenReplyReceipt(reply_id: UUID())
    }
    func model(status: String = "ready") -> ScreenModel {
        let model = ScreenModel(client: self, token: { "fixture-owner" })
        model.session = receipt
        model.active = true
        model.connected = true
        model.response.apply(ScreenEvent(type: .snapshot, seq: 0,
            text: nil, status: status, captured_at: nil))
        return model
    }
    func release() { pending?.resume(); pending = nil }
    func waitForRequest() async {
        for _ in 0..<1000 where pending == nil { await Task.yield() }
    }
    func event(_ model: ScreenModel, type: ScreenEvent.Kind, seq: Int, status: String? = nil) {
        model.receiveScreenEvent(ScreenEvent(type: type, seq: seq,
            text: nil, status: status, captured_at: nil))
    }
}