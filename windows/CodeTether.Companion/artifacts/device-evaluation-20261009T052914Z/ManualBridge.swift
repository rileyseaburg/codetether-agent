import XCTest

struct ManualCommand: Decodable {
    let id: Int
    let action: String
}

/// A bounded, manually driven connection to the already running phone app.
final class ManualBridge: XCTestCase {
    let app = XCUIApplication(bundleIdentifier: "run.codetether.ios")
    // Observation-only continuation: command 13 already sent the single request.
    var submitted = true
    var prepared = true
    func testManualConnection() throws {
        continueAfterFailure = false
        let directory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let deadline = Date().addingTimeInterval(900)
        var lastID = 0
        print("MANUAL_BRIDGE ready; no owner application launch requested")
        while Date() < deadline {
            Thread.sleep(forTimeInterval: 0.3)
            guard let bytes = try? Data(contentsOf: directory.appendingPathComponent("command.json")),
                  let command = try? JSONDecoder().decode(ManualCommand.self, from: bytes),
                  command.id > lastID else { continue }
            lastID = command.id
            if command.action == "disconnect" { break }
            let message = perform(command.action)
            let result = ManualState(id: command.id, message: message, bridge: self)
            let encoded = try JSONEncoder().encode(result)
            try encoded.write(to: directory.appendingPathComponent("response-\(command.id).json"), options: .atomic)
            print("MANUAL_BRIDGE command=\(command.id) action=\(command.action) result=\(message)")
        }
        // No app termination, relay Stop, or pairing cleanup: ownership stays with user.
    }
}