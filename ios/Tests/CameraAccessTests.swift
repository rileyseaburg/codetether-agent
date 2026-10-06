import AVFoundation
import XCTest
@testable import CodeTether

final class CameraAccessTests: XCTestCase {
    func testUnavailableCameraDoesNotRequestPermission() async {
        let result = await CameraAccess.resolve(available: false, authorization: .notDetermined) {
            XCTFail("Unavailable hardware must not prompt for permission")
            return true
        }
        XCTAssertEqual(result, .unavailable)
    }
    func testExistingAuthorizationDoesNotRequestAgain() async {
        for (status, expected) in [(AVAuthorizationStatus.authorized, CameraAccess.ready),
                                   (.denied, .denied), (.restricted, .restricted)] {
            let result = await CameraAccess.resolve(available: true, authorization: status) {
                XCTFail("Existing authorization must not prompt again")
                return false
            }
            XCTAssertEqual(result, expected)
        }
    }
    func testNewPermissionGrantAndDenial() async {
        let granted = await CameraAccess.resolve(available: true, authorization: .notDetermined) { true }
        let denied = await CameraAccess.resolve(available: true, authorization: .notDetermined) { false }
        XCTAssertEqual(granted, .ready)
        XCTAssertEqual(denied, .denied)
    }
    func testFailureStatesExplainFallback() {
        for state in [CameraAccess.denied, .restricted, .unavailable] {
            XCTAssertTrue(state.message.contains("Photos"))
        }
    }
}