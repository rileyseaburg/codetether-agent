import XCTest
@testable import CodeTether

final class InlineImageProjectionTests: XCTestCase {
    private func project(_ rows: String) throws -> [ChatMessage] {
        let json = "{\"id\":\"s\",\"messages\":[\(rows)]}"
        return try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8)).transcript
    }

    func testToolImagesStayWithTheirTurnEvenWhenMentionedAgain() throws {
        let messages = try project(#"""
        {"role":"user","content":[{"type":"text","text":"First"}]},
        {"role":"assistant","content":[{"type":"text","text":"Working"}]},
        {"role":"tool","content":[{"type":"tool_result","content":"/home/riley/first.png"}]},
        {"role":"assistant","content":[{"type":"text","text":"Done /home/riley/first.png"}]},
        {"role":"user","content":[{"type":"text","text":"Next"}]},
        {"role":"tool","content":[{"type":"tool_result","content":"/home/riley/second.png"}]},
        {"role":"assistant","content":[{"type":"text","text":"Compare /home/riley/first.png /home/riley/second.png"}]}
        """#)
        XCTAssertEqual(messages.count, 4)
        XCTAssertEqual(messages.map(\.imagePaths), [[], ["/home/riley/first.png"], [], ["/home/riley/second.png"]])
        XCTAssertEqual(messages[1].content, "Done /home/riley/first.png")
    }

    func testUploadedImageBelongsToUserNotLaterToolEcho() throws {
        let messages = try project(#"""
        {"role":"user","content":[{"type":"text","text":"Look\n\nUser attached image files: [\"/home/riley/upload.png\"]"}]},
        {"role":"tool","content":[{"type":"tool_result","content":"/home/riley/upload.png"}]},
        {"role":"assistant","content":[{"type":"text","text":"A screenshot"}]}
        """#)
        XCTAssertEqual(messages.map(\.imagePaths), [["/home/riley/upload.png"], []])
        XCTAssertEqual(messages[0].content, "Look")
    }

    func testImageOnlyToolResultDoesNotDisappearOrLeakIntoNextTurn() throws {
        let messages = try project(#"""
        {"role":"user","content":[{"type":"text","text":"Draw"}]},
        {"role":"tool","content":[{"type":"tool_result","content":"/home/riley/only.png"}]},
        {"role":"user","content":[{"type":"text","text":"Thanks"}]}
        """#)
        XCTAssertEqual(messages.map(\.role), ["user", "assistant", "user"])
        XCTAssertEqual(messages.map(\.imagePaths), [[], ["/home/riley/only.png"], []])
    }
}