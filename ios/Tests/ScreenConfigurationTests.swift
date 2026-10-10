import XCTest
@testable import CodeTether

@MainActor
final class ScreenConfigurationTests: XCTestCase {
    func testRESTCannotShortenScreenStreamLifetime() {
        let input = URLSessionConfiguration.ephemeral
        input.timeoutIntervalForResource = 777
        let client = ScreenClient(configuration: input)
        let stream = client.makeSession()
        defer { stream.invalidateAndCancel() }
        XCTAssertEqual(input.timeoutIntervalForResource, 777)
        XCTAssertEqual(stream.configuration.timeoutIntervalForResource, 3900)
        input.timeoutIntervalForResource = 1
        XCTAssertEqual(client.configuration.timeoutIntervalForResource, 3900)
    }
}
