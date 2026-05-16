// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "VisionKitBridge",
    platforms: [
        .macOS("13.0")
    ],
    products: [
        .library(
            name: "VisionKitBridge",
            type: .static,
            targets: ["VisionKitBridge"])
    ],
    targets: [
        .target(
            name: "VisionKitBridge",
            path: "Sources/VisionKitBridge",
            publicHeadersPath: "include")
    ]
)
