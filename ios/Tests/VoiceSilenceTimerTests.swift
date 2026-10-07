import XCTest
@testable import CodeTether

@MainActor
final class VoiceSilenceTimerTests: XCTestCase {
    func testThreeSecondDeadlineRestartsAfterEveryWord() {
        let timer = VoiceSilenceTimer()
        var time: TimeInterval = 0, count = 0
        timer.now = { time }; timer.onExpired = { count += 1 }
        timer.wordHeard()
        time = 2.9; timer.fireIfExpired(); XCTAssertEqual(count, 0)
        timer.wordHeard()
        time = 5.8; timer.fireIfExpired(); XCTAssertEqual(count, 0)
        time = 6; timer.fireIfExpired(); XCTAssertEqual(count, 1)
        timer.fireIfExpired(); XCTAssertEqual(count, 1)
    }

    func testFiveSecondOptionAndCancel() {
        let timer = VoiceSilenceTimer()
        var time: TimeInterval = 0, count = 0
        timer.interval = 5; timer.now = { time }; timer.onExpired = { count += 1 }
        timer.wordHeard()
        time = 3; timer.fireIfExpired(); XCTAssertEqual(count, 0)
        time = 5; timer.fireIfExpired(); XCTAssertEqual(count, 1)
        timer.wordHeard(); timer.cancel()
        time = 12; timer.fireIfExpired(); XCTAssertEqual(count, 1)
    }

    func testRealTimerExpiresWithoutAButtonPress() async {
        let timer = VoiceSilenceTimer()
        let expired = expectation(description: "Automatic deadline")
        timer.interval = 0.05
        timer.onExpired = { expired.fulfill() }
        timer.wordHeard()
        await fulfillment(of: [expired], timeout: 2)
    }
}

