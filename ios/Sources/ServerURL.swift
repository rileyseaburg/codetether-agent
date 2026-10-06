import Foundation

enum ServerURL {
    static func path(_ path: String) throws -> URL {
        guard let url = URL(string: path, relativeTo: ServerClient.origin)?.absoluteURL,
              url.scheme == "https", url.host == ServerClient.origin.host,
              url.port == nil, url.user == nil, url.password == nil, url.fragment == nil else {
            throw ClientError.invalidResponse
        }
        return url
    }
}
