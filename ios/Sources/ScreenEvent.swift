import Foundation

/// The relay's bounded event surface; unknown event types fail closed.
struct ScreenEvent: Decodable {
    enum Kind: String, Decodable { case snapshot, capture, delta, done, error, stopped }
    let type: Kind
    let seq: Int
    let text: String?
    let status: String?
    let captured_at: String?
}
