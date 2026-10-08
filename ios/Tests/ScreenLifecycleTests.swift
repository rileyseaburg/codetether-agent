import XCTest
@testable import CodeTether

@MainActor
final class ScreenLifecycleTests: XCTestCase {
    func testBackgroundCancelsStreamAndForegroundReconnects() async throws {
        let network = ScreenNetworkFixture()
        let model = ScreenModel(client: network, token: { "fixture" })
        model.setActive(true)
        await model.start(model: "fixture/vision")
        for _ in 0..<100 where !model.connected { await Task.yield() }
        XCTAssertTrue(model.connected)
        model.setActive(false)
        await model.streamTask?.value
        XCTAssertEqual(network.cancellations, 1)
        XCTAssertNotNil(model.session)
        model.setActive(true)
        for _ in 0..<100 where !model.connected { await Task.yield() }
        XCTAssertEqual(network.streams, 2)
        XCTAssertEqual(model.response.text, "Fixture analysis")
        await model.stop()
        XCTAssertNil(model.session)
        await model.streamTask?.value
    }
    func testFailedStopRetainsSessionForRetry() async {
        let network = ScreenNetworkFixture()
        let model = ScreenModel(client: network, token: { "fixture" })
        await model.start(model: "fixture/vision")
        network.stopFails = true
        await model.stop()
        XCTAssertEqual(model.session?.id, network.receipt.id)
        XCTAssertNotNil(model.error)
        network.stopFails = false
        await model.stop()
        XCTAssertNil(model.session)
    }
}