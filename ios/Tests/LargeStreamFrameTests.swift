import XCTest
@testable import CodeTether

final class LargeStreamFrameTests: XCTestCase {
    func testLargeInlineImageMetadataIsNotRetainedByFrame() async throws {
        let embedded = String(repeating: "A", count: 1024 * 1024)
        let json = "{\"type\":\"event\",\"event\":{\"kind\":\"tool.metadata\",\"payload\":{\"name\":\"image_gen\",\"metadata\":{\"saved_path\":\"/home/riley/test.png\",\"image_data_url\":{\"data_url\":\"data:image/png;base64,\(embedded)\"}}}}}"
        let decoder = AgentFrameDecoder()
        for _ in 0..<25 {
            let frame = try await decoder.decode(Data(json.utf8))
            XCTAssertEqual(frame.event?.payload?.metadata?.saved_path, "/home/riley/test.png")
            XCTAssertNil(frame.event?.payload?.text)
        }
    }
    func testOversizedIgnoredDeltaDoesNotReachUIModel() async throws {
        let text = String(repeating: "x", count: 1024 * 1024)
        let json = "{\"type\":\"event\",\"event\":{\"kind\":\"item.delta\",\"payload\":{\"text\":\"\(text)\"}}}"
        let frame = try await AgentFrameDecoder().decode(Data(json.utf8))
        XCTAssertEqual(frame.event?.kind, "item.delta")
        XCTAssertNil(frame.event?.payload)
    }
}
