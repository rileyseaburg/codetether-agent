import SwiftUI
import MarkdownUI

/// File images are rendered by their owning bubble, not reloaded from Markdown references.
/// Remote image references are links: rendering a response must not fetch tracking URLs.
struct ChatMarkdownImages: ImageProvider, InlineImageProvider {
    func image(with url: URL, label: String) async throws -> Image {
        Image(systemName: "photo")
    }
    func makeImage(url: URL?) -> some View {
        if let url, ["https", "http"].contains(url.scheme?.lowercased() ?? "") {
            Link("Open image", destination: url).font(.caption)
        } else {
            Text(url?.lastPathComponent ?? "Image attachment")
                .font(.caption).foregroundStyle(.secondary)
        }
    }
}