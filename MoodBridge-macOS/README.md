# MoodBridge macOS App

A beautiful native macOS application for the MoodBridge Legal Tech Dashboard, built with SwiftUI.

## Features

🎨 **Beautiful Design**
- Modern macOS design with gradient branding
- Native SwiftUI interface with smooth animations
- Sidebar navigation with MoodBridge branding
- Professional color scheme (blue, purple, pink gradients)

📊 **Dashboard View**
- Quick stats overview (projects, tasks, time tracking)
- Recent activity feed
- Quick action buttons
- Active work session monitoring

📁 **Project Management**
- Create and manage legal projects
- Visual project cards with progress tracking
- Priority and status indicators
- Task count and completion metrics

✅ **Task Management**
- Create and assign tasks to projects
- Task filtering (All, To Do, In Progress, Review, Completed)
- Priority and status management
- Time estimation and tracking
- Start/stop work sessions

📈 **Analytics & Insights**
- Productivity metrics and trends
- Task completion rates
- Project overview analytics
- Visual productivity charts
- Recent task activity

## Prerequisites

- macOS 13.0+ (Ventura)
- Swift 5.8+
- MoodBridge Rust API running on `http://127.0.0.1:8000`

## Building & Running

### 1. Start the MoodBridge Rust API

First, make sure your MoodBridge Rust API is running:

```bash
cd /path/to/moodbridge_rust
cargo run
```

The API should be accessible at `http://127.0.0.1:8000`

### 2. Build the macOS App

```bash
cd MoodBridge-macOS
swift build
```

### 3. Run the App

```bash
swift run MoodBridge
```

Or use the provided build script:

```bash
./build_and_run.sh
```

## App Structure

```
MoodBridge-macOS/
├── Package.swift           # Swift Package Manager configuration
├── Sources/                # Swift source files
│   ├── main.swift         # App entry point
│   ├── ContentView.swift  # Main app interface with sidebar
│   ├── Models.swift       # Data models matching Rust API
│   ├── APIService.swift   # Network layer with Alamofire
│   ├── DashboardViewModel.swift # State management
│   ├── DashboardView.swift     # Dashboard tab view
│   ├── ProjectsView.swift      # Projects management view
│   ├── TasksView.swift         # Tasks management view
│   └── AnalyticsView.swift     # Analytics and insights view
├── Resources/             # App resources (icons, assets)
└── README.md             # This file
```

## Features Explained

### 🔌 API Integration
- Full integration with MoodBridge Rust API
- Real-time data synchronization
- Connection status monitoring
- Error handling and recovery

### 🎨 MoodBridge Branding
- Brain icon with gradient styling (blue → purple → pink)
- "MoodBridge" title with gradient text
- "Legal Tech Dashboard" subtitle
- Consistent color scheme throughout

### 📱 Native macOS Experience
- NavigationSplitView for modern macOS layout
- System colors and materials
- Native form controls and pickers
- Keyboard shortcuts and accessibility
- Proper window management

## API Endpoints Used

- `GET /api/health` - Connection status
- `GET /api/dashboard-data` - Dashboard overview
- `GET /api/projects` - Project list
- `POST /api/projects` - Create project
- `GET /api/tasks` - Task list
- `POST /api/tasks` - Create task
- `PUT /api/tasks/:id` - Update task
- `POST /api/work-sessions/:task_id/start` - Start work session
- `PUT /api/work-sessions/:session_id/end` - End work session
- `GET /api/task-analytics` - Analytics data
- `GET /api/project-dashboard` - Project dashboard

## Customization

### Colors & Branding
Edit the gradient colors in `ContentView.swift`:

```swift
.foregroundStyle(
    LinearGradient(
        colors: [.blue, .purple, .pink], // Customize these colors
        startPoint: .topLeading,
        endPoint: .bottomTrailing
    )
)
```

### API Base URL
Modify the base URL in `APIService.swift`:

```swift
private let baseURL = "http://127.0.0.1:8000" // Change this
```

## Troubleshooting

**App won't connect to API:**
1. Ensure MoodBridge Rust API is running on port 8000
2. Check that no firewall is blocking the connection
3. Verify the API base URL in `APIService.swift`

**Build errors:**
1. Make sure you have Swift 5.8+ installed
2. Check that all dependencies are resolved: `swift package resolve`
3. Clean and rebuild: `swift package clean && swift build`

**Missing data:**
1. Check API connection status in the sidebar
2. Try the "Reconnect" button if disconnected
3. Use pull-to-refresh in any view to reload data

## Future Enhancements

- [ ] Menu bar app option
- [ ] Notifications for task deadlines
- [ ] Keyboard shortcuts
- [ ] Export reports to PDF
- [ ] Dark mode optimization
- [ ] Multiple API environment support
- [ ] Offline mode with CoreData

## License

This project is part of the MoodBridge Legal Tech suite.
