import XCTest
@testable import CodeTether

final class ChatClientTests: XCTestCase {
    private struct Payload: Decodable {
        let model: String
        let messages: [ChatMessage]
        let stream: Bool
    }
    func testChatPostsHistoryAndDecodesReply() async throws {
        StubProtocol.handler = { request in
            XCTAssertEqual(request.httpMethod, "POST")
            XCTAssertEqual(request.url?.path, "/v1/chat/completions")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Authorization"), "Bearer fixture-chat")
            XCTAssertEqual(request.value(forHTTPHeaderField: "Content-Type"), "application/json")
            let stream = try XCTUnwrap(request.httpBodyStream)
            stream.open()
            defer { stream.close() }
            var buffer = [UInt8](repeating: 0, count: 8192)
            let count = stream.read(&buffer, maxLength: buffer.count)
            let payload = try JSONDecoder().decode(Payload.self, from: Data(buffer.prefix(count)))
            XCTAssertEqual(payload.model, "provider/model")
            XCTAssertEqual(payload.messages.map(\.role), ["user", "assistant", "user"])
            XCTAssertFalse(payload.stream)
            return (200, Data(#"{"model":"model","choices":[{"message":{"content":"Hello Riley"}}]}"#.utf8))
        }
        let messages = [ChatMessage(role: "user", content: "Hi"),
                        ChatMessage(role: "assistant", content: "Hello"), ChatMessage(role: "user", content: "Continue")]
        let reply: ChatResponse = try await stubClient().post("v1/chat/completions", token: "fixture-chat",
                                                            body: ChatRequest(model: "provider/model", messages: messages))
        XCTAssertEqual(reply.choices.first?.message.content, "Hello Riley")
    }
}
