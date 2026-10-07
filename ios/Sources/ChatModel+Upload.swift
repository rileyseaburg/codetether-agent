import Foundation

/// Upload pending attachments and associate images with the sending user bubble.
extension ChatModel {
    func attachmentPrompt(_ text: String, token: String, turn: UUID) async throws -> String {
        let userID = messages.last(where: { $0.role == "user" })?.id
        var imagePaths: [String] = []
        var documentPaths: [String] = []
        for index in attachments.indices {
            try requireTurn(turn)
            if attachments[index].serverPath == nil {
                let uploaded: UploadedImage = try await client.post("mobile/attachments", token: token,
                    body: ImageUpload(data: attachments[index].data.base64EncodedString()))
                try requireTurn(turn)
                attachments[index].serverPath = uploaded.path
            }
            if let path = attachments[index].serverPath {
                if attachments[index].kind == .document { documentPaths.append(path) } else { imagePaths.append(path) }
            }
        }
        if let index = messages.firstIndex(where: { $0.id == userID }) {
            messages[index].imagePaths = imagePaths
        }
        return try AttachmentPrompt.build(text, imagePaths: imagePaths, documentPaths: documentPaths)
    }
}