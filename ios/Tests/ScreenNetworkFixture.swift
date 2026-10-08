import Foundation
@testable import CodeTether

@MainActor
final class ScreenNetworkFixture: ScreenNetworking {
    var stopFails = false
    var streams = 0
    var cancellations = 0
    var creates = 0
    let receipt = ScreenSession(id: UUID(), code: "ABCDEF123456",
        pair_expires_at: "2099-01-01T00:00:00Z", expires_at: "2099-01-01T01:00:00Z", interval_seconds: 30)
    func create(_ body: ScreenCreateBody, token: String) async throws -> ScreenSession {
        creates += 1; return receipt
    }
    func stop(_ id: UUID, token: String) async throws {
        if stopFails { throw ScreenFailure.http(500) }
    }
    func events(_ id: UUID, token: String, receive: @escaping @MainActor (ScreenEvent) -> Void) async throws {
        streams += 1
        receive(ScreenEvent(type: .snapshot, seq: 0, text: "Fixture analysis", status: "ready", captured_at: nil))
        do { try await Task.sleep(nanoseconds: 60_000_000_000) }
        catch { cancellations += 1; throw error }
    }
}