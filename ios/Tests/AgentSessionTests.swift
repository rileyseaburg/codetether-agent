import XCTest
@testable import CodeTether

final class AgentSessionTests: XCTestCase {
    func testTranscriptKeepsTextAndToolsButNotReasoning() throws {
        let json = #"{"id":"s","messages":[{"role":"user","content":[{"type":"text","text":"hello"}]},{"role":"assistant","content":[{"type":"thinking","text":"private reasoning"},{"type":"tool_call","name":"websearch"},{"type":"text","text":"reply"}]}]}"#
        let session = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        XCTAssertEqual(session.transcript.map(\.content), ["hello", "reply"])
        XCTAssertEqual(session.toolNames, ["websearch"])
    }
    func testPaginationUsesRealQueryNotEscapedPath() throws {
        let url = try ServerURL.path("api/session?limit=50&offset=50")
        XCTAssertEqual(url.path, "/api/session")
        XCTAssertEqual(url.query, "limit=50&offset=50")
    }
    func testForeignOriginAndCredentialsAreRejected() {
        XCTAssertThrowsError(try ServerURL.path("https://example.invalid/api/session"))
        XCTAssertThrowsError(try ServerURL.path("http://server.codetether.run/api/session"))
        XCTAssertThrowsError(try ServerURL.path("https://user:pass@server.codetether.run/api/session"))
    }
    @MainActor
    func testStreamedToolsAndImagesAreVisibleBeforeTurnFinishes() throws {
        let chat = ChatModel()
        let event = #"{"kind":"tool.metadata","payload":{"name":"image_gen","metadata":{"saved_path":"/home/riley/image.png"}}}"#
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(event.utf8)))
        XCTAssertEqual(chat.messages.last?.imagePaths, ["/home/riley/image.png"])
        let done = #"{"kind":"tool.completed","payload":{"name":"websearch","success":true}}"#
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(done.utf8)))
        XCTAssertEqual(chat.tools, ["websearch"])
    }
    @MainActor
    func testAssistantItemsAppearOnceBeforeTerminalResult() throws {
        let chat = ChatModel()
        let start = #"{"kind":"item.started","payload":{"item_id":"a","item_type":"assistant_text"}}"#
        let end = #"{"kind":"item.completed","payload":{"item_id":"a","text":"Hello"}}"#
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(start.utf8)))
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(end.utf8)))
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(end.utf8)))
        XCTAssertEqual(chat.messages.map(\.content), ["Hello"])
        XCTAssertEqual(chat.replyForSpeech, "Hello")
    }
}