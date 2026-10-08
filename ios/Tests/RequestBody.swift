import Foundation
import XCTest

/// URLProtocol receives upload data either directly or through an input stream.
enum RequestBody {
    static func data(_ request: URLRequest) throws -> Data {
        if let body = request.httpBody { return body }
        let stream = try XCTUnwrap(request.httpBodyStream)
        stream.open(); defer { stream.close() }
        var result = Data(), buffer = [UInt8](repeating: 0, count: 4096)
        while true {
            let count = stream.read(&buffer, maxLength: buffer.count)
            if count == 0 { return result }
            guard count > 0 else { throw ClientBodyError.read }
            result.append(contentsOf: buffer.prefix(count))
        }
    }
    private enum ClientBodyError: Error { case read }
}