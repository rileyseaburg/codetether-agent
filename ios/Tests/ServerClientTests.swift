import XCTest
@testable import CodeTether

final class ServerClientTests: XCTestCase {
    func testVersionUsesAuthenticatedHTTPS() async throws {
        StubProtocol.handler = { request in
            XCTAssertEqual(request.url?.absoluteString, "https://server.codetether.run/api/version")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Authorization"), "Bearer fixture-token")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Accept"), "application/json")
            return (200, Data(#"{"version":"4.7.5","name":"codetether-agent"}"#.utf8))
        }
        let version: ServerVersion = try await stubClient().get("api/version", token: "fixture-token")
        XCTAssertEqual(version.version, "4.7.5")
    }

    func testAgentContractAllowsNullDescriptionAndExtraFields() async throws {
        StubProtocol.handler = { request in
            XCTAssertEqual(request.url?.path, "/api/agent")
            return (200, Data(#"[{"name":"build","mode":"primary","description":null,"hidden":false,"native":true}]"#.utf8))
        }
        let agents: [AgentProfile] = try await stubClient().get("api/agent", token: "fixture")
        XCTAssertEqual(agents.first?.id, "build")
        XCTAssertNil(agents.first?.description)
    }

    func testMissingTokenNeverSendsRequest() async {
        StubProtocol.handler = { _ in XCTFail("Must not send"); return (200, Data()) }
        do {
            let _: ServerVersion = try await stubClient().get("api/version", token: " \n")
            XCTFail("Expected missing token")
        } catch { XCTAssertEqual(error.localizedDescription, ClientError.missingToken.localizedDescription) }
    }

    func testHeaderInjectionNeverSendsRequest() async {
        StubProtocol.handler = { _ in XCTFail("Must not send"); return (200, Data()) }
        do {
            let _: ServerVersion = try await stubClient().get("api/version", token: "a\r\nX-Evil: b")
            XCTFail("Expected invalid token")
        } catch { XCTAssertEqual(error.localizedDescription, ClientError.missingToken.localizedDescription) }
    }
}
