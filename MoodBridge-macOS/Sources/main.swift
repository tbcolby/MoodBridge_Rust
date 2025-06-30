import SwiftUI
import AppKit

class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Create the SwiftUI view
        let contentView = ContentView()
        
        // Create the window and set the content view
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 1200, height: 800),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        window.center()
        window.setFrameAutosaveName("MoodBridge")
        window.contentView = NSHostingView(rootView: contentView)
        window.title = "MoodBridge - Legal Tech Dashboard"
        window.makeKeyAndOrderFront(nil)
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
    
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
}

// Entry point
let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.run()
