import XCTest
@testable import CodeTether

final class StreamMemoryTests: XCTestCase {
    private func event(_ text: String) throws -> AgentFrame.Event {
        try JSONDecoder().decode(AgentFrame.Event.self, from: Data(text.utf8))
    }
    @MainActor
    func testTokenFloodDoesNotPublishOrRetainText() throws {
        let chat = ChatModel()
        let token = try event(#"{"kind":"item.delta","payload":{"text":{"unexpected":"ignored"}}}"#)
        XCTAssertNil(token.payload)
        for _ in 0..<100_000 { chat.receive(token) }
        XCTAssertTrue(chat.messages.isEmpty)
        XCTAssertTrue(chat.completedItems.isEmpty)
        XCTAssertNil(chat.activeAssistantItem)
    }
    @MainActor
    func testCompletedEventFloodStaysOneBubbleAndBoundedDedup() throws {
        let chat = ChatModel()
        for number in 0..<2000 {
            chat.receive(try event("{\"kind\":\"item.started\",\"payload\":{\"item_type\":\"assistant_text\",\"item_id\":\"\(number)\"}}"))
            chat.receive(try event("{\"kind\":\"item.completed\",\"payload\":{\"item_id\":\"\(number)\",\"text\":\"Reply \(number)\"}}"))
        }
        XCTAssertEqual(chat.messages.count, 1)
        XCTAssertEqual(chat.messages.first?.content, "Reply 1999")
        XCTAssertEqual(chat.completedItems.count, 128)
        let id = chat.messages.first?.id
        chat.upsertReply("Final answer")
        XCTAssertEqual(chat.messages.count, 1)
        XCTAssertEqual(chat.messages.first?.id, id)
    }
    func testSessionDecoderDropsReasoningAndLargeToolLogs() throws {
        let json = #"{"id":"s","messages":[{"role":"assistant","content":[{"type":"thinking","text":"private"}]},{"role":"tool","content":[{"type":"tool_result","content":"large tool output"}]}]}"#
        let session = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        XCTAssertNil(session.messages?.first?.content.first?.text)
        XCTAssertEqual(session.messages?.last?.content.first?.content, "")
    }
    func testIgnoredFrameDecodeMemory() {
        let frame = Data(#"{"type":"event","event":{"kind":"item.delta","payload":{"text":"ignored"}}}"#.utf8)
        measure(metrics: [XCTMemoryMetric()]) {
            for _ in 0..<10_000 { _ = try? JSONDecoder().decode(AgentFrame.self, from: frame) }
        }
    }
}
