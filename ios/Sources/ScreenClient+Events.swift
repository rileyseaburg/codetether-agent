import Foundation

extension ScreenClient {
    func events(_ id: UUID, token: String,
                receive: @escaping @MainActor (ScreenEvent) -> Void) async throws {
        let session = makeSession()
        defer { session.invalidateAndCancel() }
        var request = try ScreenHTTP.request("/companion/sessions/\(id.uuidString.lowercased())/events", token: token)
        request.setValue("text/event-stream", forHTTPHeaderField: "Accept")
        try await withTaskCancellationHandler {
            let (bytes, response) = try await session.bytes(for: request)
            try ScreenHTTP.validate(response, stream: true)
            var decoder = ScreenSSEDecoder()
            var awaitingSnapshot = true
            for try await byte in bytes {
                try Task.checkCancellation()
                guard let event = try decoder.consume(byte) else { continue }
                if awaitingSnapshot {
                    guard event.type == .snapshot else { throw ScreenFailure.invalidEvent }
                    awaitingSnapshot = false
                }
                receive(event)
                if event.type == .stopped { return }
            }
            throw ScreenFailure.disconnected
        } onCancel: {
            session.invalidateAndCancel()
        }
    }
}