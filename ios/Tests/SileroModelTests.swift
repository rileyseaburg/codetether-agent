import XCTest
@testable import CodeTether

/// Executes the bundled model itself, not a fake probability producer.
@MainActor
final class SileroModelTests: XCTestCase {
    func testBundledModelRunsAndResetsWithTheActualTensorContract() throws {
        let model = try SileroVAD()
        let frame = [Float](repeating: 0, count: 4096)
        let first = try model.process(frame)
        XCTAssertTrue(first.isFinite)
        XCTAssertGreaterThanOrEqual(first, 0); XCTAssertLessThanOrEqual(first, 1)
        for _ in 0..<3 { XCTAssertTrue(try model.process(frame).isFinite) }
        model.reset()
        XCTAssertEqual(try model.process(frame), first, accuracy: 0.0001)
    }

    func testWrongFrameSizeIsRejected() throws {
        let model = try SileroVAD()
        XCTAssertThrowsError(try model.process([0, 0]))
    }
}

