import XCTest
@testable import CodeTether

final class TokenStorageTests: XCTestCase {
    override func tearDownWithError() throws {
        try TokenStore.remove()
        let file = URL.documentsDirectory.appendingPathComponent("bootstrap.json")
        if FileManager.default.fileExists(atPath: file.path) {
            try FileManager.default.removeItem(at: file)
        }
    }

    func testKeychainCreateReplaceAndRemove() throws {
        try TokenStore.remove()
        XCTAssertNil(try TokenStore.read())
        try TokenStore.save("fixture-first")
        XCTAssertEqual(try TokenStore.read(), "fixture-first")
        try TokenStore.save("fixture-replacement")
        XCTAssertEqual(try TokenStore.read(), "fixture-replacement")
        try TokenStore.remove()
        XCTAssertNil(try TokenStore.read())
    }

    func testBootstrapConsumesFileIntoKeychain() throws {
        let file = URL.documentsDirectory.appendingPathComponent("bootstrap.json")
        try Data(#"{"token":"fixture-bootstrap"}"#.utf8).write(to: file)
        try TokenBootstrap.consume()
        XCTAssertEqual(try TokenStore.read(), "fixture-bootstrap")
        XCTAssertFalse(FileManager.default.fileExists(atPath: file.path))
    }

    func testMalformedBootstrapIsRemovedWithoutReplacingToken() throws {
        try TokenStore.save("fixture-existing")
        let file = URL.documentsDirectory.appendingPathComponent("bootstrap.json")
        try Data(#"{"token":"a\r\nb"}"#.utf8).write(to: file)
        XCTAssertThrowsError(try TokenBootstrap.consume())
        XCTAssertEqual(try TokenStore.read(), "fixture-existing")
        XCTAssertFalse(FileManager.default.fileExists(atPath: file.path))
    }
}
