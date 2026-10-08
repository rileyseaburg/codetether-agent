import XCTest
@testable import CodeTether

final class ScreenSSETests: XCTestCase {
    func testFragmentedUTF8AndHeartbeat() throws {
        var decoder = ScreenSSEDecoder()
        let wire = ": heartbeat\r\n\r\ndata: {\"type\":\"delta\",\"seq\":1,\"text\":\"héllo\"}\r\n\r\n"
        var events: [ScreenEvent] = []
        for byte in wire.utf8 {
            if let event = try decoder.consume(byte) { events.append(event) }
        }
        XCTAssertEqual(events.count, 1)
        XCTAssertEqual(events.first?.text, "héllo")
    }
    func testSnapshotReplacesRatherThanDuplicatesPartialText() {
        var state = ScreenResponse()
        state.apply(ScreenEvent(type: .capture, seq: 1, text: nil, status: "analyzing", captured_at: nil))
        state.apply(ScreenEvent(type: .delta, seq: 2, text: "Hello", status: nil, captured_at: nil))
        state.apply(ScreenEvent(type: .snapshot, seq: 2, text: "Hello", status: "analyzing", captured_at: nil))
        state.apply(ScreenEvent(type: .delta, seq: 2, text: "duplicate", status: nil, captured_at: nil))
        state.apply(ScreenEvent(type: .delta, seq: 3, text: " world", status: nil, captured_at: nil))
        XCTAssertEqual(state.text, "Hello world")
    }
    func testUnknownEventsAndInvalidCredentialsFailClosed() throws {
        var parser = ScreenSSEParser()
        _ = try parser.consume("data: {\"type\":\"command\",\"seq\":0}")
        XCTAssertThrowsError(try parser.consume(""))
        XCTAssertThrowsError(try ScreenHTTP.request("/companion/sessions", token: "bad\ntoken"))
        let request = try ScreenHTTP.request("/companion/sessions", token: "fixture")
        XCTAssertNil(request.url?.query)
        XCTAssertEqual(request.value(forHTTPHeaderField: "Authorization"), "Bearer fixture")
        XCTAssertEqual(request.url?.host, "server.codetether.run")
    }
}