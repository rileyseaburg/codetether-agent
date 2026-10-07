import XCTest
@testable import CodeTether

final class ModelSelectionHandshakeTests: XCTestCase {
    func testSelectedModelRequiresExplicitServerSupport() throws {
        let legacy = try JSONDecoder().decode(AgentFrame.self, from: Data(#"{"type":"ready"}"#.utf8))
        let supported = try JSONDecoder().decode(AgentFrame.self,
            from: Data(#"{"type":"ready","model_selection":true}"#.utf8))
        let command = AgentPromptCommand(message: "Hello", model: "provider/chosen")
        XCTAssertThrowsError(try command.validateServer(legacy))
        XCTAssertNoThrow(try command.validateServer(supported))
    }

    func testDefaultModelRemainsCompatibleWithLegacyServers() throws {
        let ready = try JSONDecoder().decode(AgentFrame.self, from: Data(#"{"type":"ready"}"#.utf8))
        XCTAssertNoThrow(try AgentPromptCommand(message: "Hello", model: "").validateServer(ready))
    }
}


