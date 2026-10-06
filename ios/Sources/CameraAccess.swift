import AVFoundation
import UIKit

enum CameraAccess: Equatable {
    case ready, denied, restricted, unavailable

    static func resolve(available: Bool, authorization: AVAuthorizationStatus,
                        request: () async -> Bool) async -> CameraAccess {
        guard available else { return .unavailable }
        switch authorization {
        case .authorized: return .ready
        case .denied: return .denied
        case .restricted: return .restricted
        case .notDetermined: return await request() ? .ready : .denied
        @unknown default: return .unavailable
        }
    }

    @MainActor static func request() async -> CameraAccess {
        await resolve(available: UIImagePickerController.isSourceTypeAvailable(.camera),
                      authorization: AVCaptureDevice.authorizationStatus(for: .video)) {
            await AVCaptureDevice.requestAccess(for: .video)
        }
    }

    var message: String {
        switch self {
        case .denied: return "Allow camera access in Settings to take a photo, or use Photos."
        case .restricted: return "Camera access is restricted on this iPhone. You can use Photos instead."
        case .unavailable: return "No camera is available. You can use Photos instead."
        case .ready: return ""
        }
    }
}