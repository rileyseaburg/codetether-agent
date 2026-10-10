import XCTest
@testable import CodeTether

@MainActor
final class ScreenQuestionHTTPTests: XCTestCase {
    private func client() -> ScreenClient {
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [StubProtocol.self]
        return ScreenClient(configuration: config)
    }
    func testAskPostsOnlyQuestionToOwnerEndpointAndAccepts202() async throws {
        let id = UUID(), requestID = UUID()
        let question = "Type CT-IOS-CHECK in the focused field."
        defer { StubProtocol.handler = nil }
        StubProtocol.handler = { request in
            XCTAssertEqual(request.url?.absoluteString,
                "https://server.codetether.run/companion/sessions/\(id.uuidString.lowercased())/request")
            XCTAssertEqual(request.httpMethod, "POST")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Authorization"), "Bearer fixture-owner")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Cache-Control"), "no-store")
            let stream = try XCTUnwrap(request.httpBodyStream)
            stream.open(); defer { stream.close() }
            var buffer = [UInt8](repeating: 0, count: 8192)
            let count = stream.read(&buffer, maxLength: buffer.count)
            let body = try JSONDecoder().decode([String: String].self, from: Data(buffer.prefix(count)))
            XCTAssertEqual(body, ["question": question])
            return (202, Data("{\"request_id\":\"\(requestID)\"}".utf8))
        }
        let receipt = try await client().ask(ScreenQuestion(question: question), session: id, token: "fixture-owner")
        XCTAssertEqual(receipt.request_id, requestID)
    }
    func testMalformedAcceptanceIsRejectedWithoutRetry() async {
        var calls = 0
        defer { StubProtocol.handler = nil }
        StubProtocol.handler = { _ in calls += 1; return (202, Data("{}".utf8)) }
        do {
            _ = try await client().ask(ScreenQuestion(question: "Describe"), session: UUID(), token: "fixture-owner")
            XCTFail("Malformed acceptance must fail")
        } catch { XCTAssertEqual(calls, 1) }
    }
    func testMissingCredentialNeverSends() async {
        defer { StubProtocol.handler = nil }
        StubProtocol.handler = { _ in XCTFail("Must not send"); return (401, Data()) }
        do {
            _ = try await client().ask(ScreenQuestion(question: "Describe"), session: UUID(), token: "")
            XCTFail("Missing credential must fail")
        } catch { XCTAssertEqual(error.localizedDescription, ClientError.missingToken.localizedDescription) }
    }
}