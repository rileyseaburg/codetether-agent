import XCTest
@testable import CodeTether

final class ResponseFailureTests: XCTestCase {
    func testHTTPFailuresDoNotExposeResponseBodies() async {
        for (status, expected) in [(401, ClientError.unauthorized), (403, .forbidden), (503, .http(503))] {
            StubProtocol.handler = { _ in (status, Data("private server details".utf8)) }
            do {
                let _: ServerVersion = try await stubClient().get("api/version", token: "fixture")
                XCTFail("Expected failure")
            } catch { XCTAssertEqual(error.localizedDescription, expected.localizedDescription) }
        }
    }

    func testMalformedJSONIsSafeError() async {
        StubProtocol.handler = { _ in (200, Data("not json".utf8)) }
        do {
            let _: ServerVersion = try await stubClient().get("api/version", token: "fixture")
            XCTFail("Expected decode failure")
        } catch { XCTAssertEqual(error.localizedDescription, ClientError.invalidResponse.localizedDescription) }
    }

    func testRedirectDelegateRefusesCredentialForwarding() {
        let session = URLSession(configuration: .ephemeral)
        let original = ServerClient.origin.appendingPathComponent("api/version")
        let response = HTTPURLResponse(url: original, statusCode: 302, httpVersion: nil,
                                       headerFields: ["Location": "https://example.invalid"])!
        let request = URLRequest(url: URL(string: "https://example.invalid")!)
        let completion = expectation(description: "Redirect refused")
        RejectRedirects().urlSession(session, task: session.dataTask(with: original),
                                     willPerformHTTPRedirection: response, newRequest: request) { next in
            XCTAssertNil(next); completion.fulfill()
        }
        wait(for: [completion], timeout: 1)
    }
}
