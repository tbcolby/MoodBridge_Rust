// swift-tools-version: 5.8
import PackageDescription

let package = Package(
    name: "MoodBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "MoodBridge", targets: ["MoodBridge"])
    ],
    dependencies: [
        .package(url: "https://github.com/Alamofire/Alamofire.git", from: "5.8.0")
    ],
    targets: [
        .executableTarget(
            name: "MoodBridge",
            dependencies: [
                "Alamofire"
            ],
            path: "Sources"
        )
    ]
)
