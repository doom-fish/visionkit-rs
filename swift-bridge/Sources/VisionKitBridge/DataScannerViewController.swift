import Foundation

@_cdecl("vk_data_scanner_view_controller_support_json")
public func vk_data_scanner_view_controller_support_json(
    _ outSupportJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        try vkWriteJSON(
            vkAreaSupportPayload(
                area: "DataScannerViewController",
                availableOnCurrentPlatform: false,
                availability: "iOS 16+; @available(macCatalyst, unavailable)",
                reason: "Apple ships DataScannerViewController, RecognizedItem, and camera-driven scanning APIs as iOS-only VisionKit surface with no macOS implementation.",
                members: [
                    "RecognizedDataType.text(languages:textContentType:)",
                    "RecognizedDataType.barcode(symbologies:)",
                    "QualityLevel.{balanced,fast,accurate}",
                    "TextContentType.{dateTimeDuration,emailAddress,flightNumber,fullStreetAddress,shipmentTrackingNumber,telephoneNumber,URL,currency}",
                    "ScanningUnavailable.{unsupported,cameraRestricted}",
                    "class var isSupported",
                    "class var isAvailable",
                    "class var supportedTextRecognitionLanguages",
                    "overlayContainerView",
                    "recognizedDataTypes",
                    "qualityLevel",
                    "recognizesMultipleItems",
                    "isHighFrameRateTrackingEnabled",
                    "isPinchToZoomEnabled",
                    "isGuidanceEnabled",
                    "isHighlightingEnabled",
                    "regionOfInterest",
                    "isScanning",
                    "recognizedItems",
                    "minZoomFactor / maxZoomFactor / zoomFactor",
                    "capturePhoto()",
                    "startScanning() / stopScanning()",
                    "delegate callbacks for tap/add/update/remove/zoom/unavailable",
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
