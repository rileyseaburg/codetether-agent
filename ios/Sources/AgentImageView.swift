import SwiftUI

struct AgentImageView: View {
    let path: String
    @State private var image: UIImage?
    @State private var failed = false
    var body: some View {
        Group {
            if let image { Image(uiImage: image).resizable().scaledToFit().frame(maxHeight: 260) }
            else if failed { Label("Image unavailable", systemImage: "photo") }
            else { ProgressView("Loading image…") }
        }.task(id: path) {
            failed = false
            do {
                guard let token = try TokenStore.read() else { throw ClientError.missingToken }
                var url = URLComponents(url: ServerClient.origin.appendingPathComponent("mobile/image"), resolvingAgainstBaseURL: false)!
                url.queryItems = [URLQueryItem(name: "path", value: path)]
                var request = URLRequest(url: url.url!)
                request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
                let session = URLSession(configuration: .ephemeral, delegate: RejectRedirects(), delegateQueue: nil)
                defer { session.invalidateAndCancel() }
                let (data, response) = try await session.data(for: request)
                try Task.checkCancellation()
                guard (response as? HTTPURLResponse)?.statusCode == 200, let decoded = ImageThumbnail.decode(data) else {
                    throw ClientError.invalidResponse
                }
                image = decoded
            } catch { if !Task.isCancelled { failed = true } }
        }.onDisappear { image = nil }
    }
}
