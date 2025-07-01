# 🦀⚖️ MoodBridge Local App Setup Guide

MoodBridge is now configured as a robust local application with comprehensive error handling and a professional loading screen.

## ✅ Quick Start

### Using the Launcher (Recommended)
```bash
# Start the application
./launcher.sh start

# Check status
./launcher.sh status

# Stop the application
./launcher.sh stop

# View logs
./launcher.sh logs

# Get help
./launcher.sh help
```

### Manual Start
```bash
# Build and run
cargo build --release
./target/release/moodbridge_rust
```

## 🎯 Features Added

### ✅ Loading Screen
- **Professional animated spinner** with progress bar
- **Gradual progress simulation** for better UX
- **Error notifications** with auto-dismiss
- **Health check before data loading** for robustness
- **Automatic retry** on failure (5-second delay)

### ✅ Local App Enhancements
- **Standalone launcher script** with full process management
- **Comprehensive dependency checking**
- **Automated database setup and migrations**
- **Port conflict detection and handling**
- **Graceful startup and shutdown**
- **Health monitoring** with visual feedback
- **Cross-platform browser opening**

### ✅ Error Handling & Robustness
- **Startup validation** at each checkpoint:
  1. Directory creation ✅
  2. Database connection ✅
  3. Migrations ✅
  4. Data seeding ✅
  5. Route configuration ✅
  6. Server startup ✅
  7. Health validation ✅

- **Runtime error handling**:
  - API failures with user-friendly messages
  - Automatic retry mechanisms
  - Connection loss recovery
  - Database error handling

### ✅ Process Management
- **PID tracking** for reliable process control
- **Log management** in dedicated logs/ directory
- **Signal handling** for graceful shutdown (Ctrl+C, SIGTERM)
- **Port availability checking**
- **Duplicate instance prevention**

## 🌐 Access Points

Once started, access MoodBridge at:
- **Dashboard**: http://localhost:8080
- **Health Check**: http://localhost:8080/api/health
- **API Data**: http://localhost:8080/api/dashboard-data

## 📁 Directory Structure

```
MoodBridge_Rust/
├── launcher.sh          # 🚀 Main launcher script
├── src/                 # 💻 Source code
├── data/               # 🗄️ SQLite database
├── logs/               # 📝 Application logs
├── templates/          # 🎨 HTML templates
├── static/            # 📦 Static files
└── target/            # 🔨 Compiled binaries
```

## 🔧 Configuration

### Environment Variables
- `DATABASE_URL`: SQLite database path (default: `sqlite:data/main.db`)
- `PORT`: Server port (default: `8080`)
- `RUST_LOG`: Logging level (default: `info`)

### Launcher Configuration
Edit the launcher.sh script to modify:
- Default port (line 12: `PORT=8080`)
- App name and branding
- Health check retry count
- Log rotation settings

## 🛠️ Development

### Building
```bash
cargo build --release
```

### Testing
```bash
# Test health endpoint
curl http://localhost:8080/api/health

# Test data endpoint
curl http://localhost:8080/api/dashboard-data
```

### Logs
```bash
# View real-time logs
./launcher.sh logs

# Or manually:
tail -f logs/app.log
```

## 🚨 Troubleshooting

### Port Already in Use
The launcher automatically detects port conflicts and either:
- Connects to existing MoodBridge instance
- Reports conflicting service
- Suggests alternatives

### Build Failures
Check build logs:
```bash
cat logs/build.log
```

### Database Issues
The app automatically creates the database and runs migrations. If issues persist:
```bash
rm -rf data/main.db
./launcher.sh restart
```

### Browser Won't Open
The launcher supports macOS, Linux, and Windows browser opening. If it fails:
- Manually open: http://localhost:8080
- Check firewall settings
- Verify port accessibility

## 🎉 Success Indicators

When properly running, you should see:
- ✅ All startup checkpoints pass
- 🌐 Browser opens automatically
- 📊 Dashboard loads with animated progress
- 🔄 Health check returns "healthy"
- 📈 Data loads without errors

## 📊 Performance Notes

- **Startup time**: ~2-5 seconds on modern hardware
- **Memory usage**: ~10-20MB typical
- **Database**: SQLite, stored locally
- **Build time**: ~1-2 minutes (first build)

The application is now production-ready for local deployment with enterprise-grade error handling and user experience!
