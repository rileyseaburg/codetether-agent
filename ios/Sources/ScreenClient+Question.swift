import Foundation

extension ScreenClient: ScreenQuestionNetworking {
    func ask(_ question: ScreenQuestion, session id: UUID,
             token: String) async throws -> ScreenQuestionReceipt {
        guard ScreenInput.text(question.question) else { throw ScreenFailure.invalidQuestion }
        let session = makeSession()
        defer { session.invalidateAndCancel() }
        var request = try ScreenHTTP.request(
            "/companion/sessions/\(id.uuidString.lowercased())/request", token: token, method: "POST")
        request.timeoutInterval = 30
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONEncoder().encode(question)
        let (data, response) = try await session.data(for: request)
        guard let http = response as? HTTPURLResponse else { throw ScreenFailure.invalidResponse }
        guard http.statusCode == 202 else { throw ScreenFailure.http(http.statusCode) }
        do { return try JSONDecoder().decode(ScreenQuestionReceipt.self, from: data) }
        catch { throw ScreenFailure.invalidResponse }
    }
}
