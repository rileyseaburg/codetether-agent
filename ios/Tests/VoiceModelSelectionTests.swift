import XCTest
@testable import CodeTether

@MainActor
final class VoiceModelSelectionTests: XCTestCase {
    func testVoiceUsesSelectedBackendModelAndCanSwitchNextTurn() async throws {
        try TokenStore.save("fixture-voice-model")
        let saved = UserDefaults.standard.string(forKey: "chat.model")
        defer { try? TokenStore.remove(); UserDefaults.standard.set(saved, forKey: "chat.model") }
        let id = UUID().uuidString
        StubProtocol.handler = { _ in (200, Data("{\"id\":\"\(id)\",\"messages\":[]}".utf8)) }
        let chat = ChatModel(client: stubClient()), agent = SessionIsolationAgent()
        chat.agent = agent; chat.sessionID = id
        chat.selectedModel = "provider/first"
        _ = await chat.sendAsync("First voice turn")
        chat.selectedModel = "provider/second"
        _ = await chat.sendAsync("Second voice turn")
        XCTAssertEqual(agent.models, ["provider/first", "provider/second"])
        XCTAssertEqual(agent.sessions, [id, id])
        XCTAssertEqual(UserDefaults.standard.string(forKey: "chat.model"), "provider/second")
    }

    func testTurnCapturesSelectionBeforeAsyncWork() throws {
        let chat = ChatModel()
        chat.selectedModel = "provider/chosen"
        let turn = try XCTUnwrap(chat.beginTurn("Hello", owner: .voice))
        chat.selectedModel = "provider/later"
        XCTAssertEqual(turn.model, "provider/chosen")
        chat.clear()
    }

    func testRealtimeCommandCarriesQualifiedModelSeparatelyFromUserText() throws {
        struct Command: Decodable { let type: String; let message: String; let model: String? }
        let data = try JSONEncoder().encode(AgentPromptCommand(message: "Hello", model: "provider/chosen"))
        let command = try JSONDecoder().decode(Command.self, from: data)
        XCTAssertEqual(command.type, "prompt"); XCTAssertEqual(command.message, "Hello")
        XCTAssertEqual(command.model, "provider/chosen")
        let defaultData = try JSONEncoder().encode(AgentPromptCommand(message: "Hello", model: ""))
        XCTAssertNil(try JSONDecoder().decode(Command.self, from: defaultData).model)
    }
}