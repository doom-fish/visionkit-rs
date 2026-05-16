import Foundation

@_cdecl("vk_recognized_text_support_json")
public func vk_recognized_text_support_json(
    _ outSupportJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        try vkWriteJSON(
            vkAreaSupportPayload(
                area: "RecognizedText",
                availableOnCurrentPlatform: false,
                availability: "iOS 16+; part of RecognizedItem.Text; @available(macCatalyst, unavailable)",
                reason: "RecognizedItem.Text is nested under DataScannerViewController's iOS-only live camera APIs and is not present in macOS VisionKit.",
                members: [
                    "id",
                    "bounds",
                    "transcript",
                    "observation",
                ],
                notes: [
                    "Use ImageAnalysis::transcript on macOS for OCR text extracted from still images.",
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
