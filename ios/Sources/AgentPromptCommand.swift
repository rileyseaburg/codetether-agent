import Foundation

/// Backend-model selection is part of the turn command, never an instruction in user text.
struct AgentPromptCommand: Encodable {
    let type = "prompt"
    let message: String
    let model: String?
    init(message: String, model: String) {
        self.message = message
        self.model = model.isEmpty ? nil : model
    }

    /// Fail visibly on older servers rather than silently ignoring the user's selection.
    func validateServer(_ frame: AgentFrame) throws {
        guard model == nil || frame.model_selection == true else {
            throw ClientError.agent("The backend needs an update to accept model selection. Your message was not sent.")
        }
    }
}