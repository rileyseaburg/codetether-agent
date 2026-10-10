import Foundation

extension ScreenClient: ScreenReplyNetworking {
    func send(_ reply: ScreenReply, session id: UUID,
              token: String) async throws -> ScreenReplyReceipt {
        guard ScreenInput.text(reply.text) else { throw ScreenFailure.invalidQuestion }
        let session = makeSession()
        defer { session.invalidateAndCancel() }
        var request = try ScreenHTTP.request(
            "/companion/sessions/\(id.uuidString.lowercased())/reply", token: token, method: "POST")
        request.timeoutInterval = 30
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONEncoder().encode(reply)
        let (data, response) = try await session.data(for: request)
        guard let http = response as? HTTPURLResponse else { throw ScreenFailure.invalidResponse }
        guard http.statusCode == 202 else { throw ScreenFailure.http(http.statusCode) }
        do { return try JSONDecoder().decode(ScreenReplyReceipt.self, from: data) }
        catch { throw ScreenFailure.invalidResponse }
    }
}
