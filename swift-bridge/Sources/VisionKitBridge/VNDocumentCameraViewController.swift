import Foundation

@_cdecl("vk_vn_document_camera_view_controller_support_json")
public func vk_vn_document_camera_view_controller_support_json(
    _ outSupportJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        try vkWriteJSON(
            vkAreaSupportPayload(
                area: "VNDocumentCameraViewController",
                availableOnCurrentPlatform: false,
                availability: "iOS 13+; API_UNAVAILABLE(macos, tvos, watchos)",
                reason: "Apple marks VNDocumentCameraViewController and its VNDocumentCameraScan result type unavailable on macOS.",
                members: [
                    "delegate",
                    "class var isSupported",
                    "delegate callback didFinishWithScan",
                    "delegate callback didCancel",
                    "delegate callback didFailWithError",
                    "VNDocumentCameraScan.pageCount",
                    "VNDocumentCameraScan.imageOfPageAtIndex(_:)",
                    "VNDocumentCameraScan.title",
                ],
                notes: [
                    "This crate exposes the area on macOS as availability metadata only.",
                ]
            ),
            to: outSupportJson
        )
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}
