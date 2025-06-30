#!/bin/bash

# MoodBridge Local App Launcher
# Ensures robust startup with error handling

set -e

APP_NAME="MoodBridge Legal Dashboard"
APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="${APP_DIR}/logs"
PID_FILE="${APP_DIR}/moodbridge.pid"
PORT=8080

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🦀⚖️ ${APP_NAME} Launcher${NC}"
echo -e "${BLUE}======================================${NC}"

# Create logs directory
mkdir -p "${LOG_DIR}"

# Function to check if port is available
check_port() {
    if lsof -Pi :${PORT} -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Port ${PORT} is already in use${NC}"
        existing_pid=$(lsof -Pi :${PORT} -sTCP:LISTEN -t)
        echo -e "${YELLOW}   Process PID: ${existing_pid}${NC}"
        
        # Check if it's our app
        if ps -p "${existing_pid}" -o comm= | grep -q "moodbridge"; then
            echo -e "${GREEN}✅ MoodBridge is already running!${NC}"
            echo -e "${GREEN}   Access it at: http://localhost:${PORT}${NC}"
            return 1
        else
            echo -e "${RED}❌ Another service is using port ${PORT}${NC}"
            echo -e "${RED}   Please stop the other service or change the port${NC}"
            return 1
        fi
    fi
    return 0
}

# Function to check dependencies
check_dependencies() {
    echo -e "${BLUE}🔍 Checking dependencies...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/Cargo is not installed${NC}"
        echo -e "${RED}   Please install Rust from https://rustup.rs/${NC}"
        exit 1
    fi
    
    if ! command -v sqlite3 &> /dev/null; then
        echo -e "${YELLOW}⚠️  SQLite3 not found in PATH (optional)${NC}"
    fi
    
    echo -e "${GREEN}✅ Dependencies OK${NC}"
}

# Function to check database and setup
check_database() {
    echo -e "${BLUE}🗄️  Checking database setup...${NC}"
    
    mkdir -p "${APP_DIR}/data"
    
    if [ ! -f "${APP_DIR}/data/main.db" ]; then
        echo -e "${YELLOW}⚠️  Database not found, will be created on startup${NC}"
    else
        echo -e "${GREEN}✅ Database found${NC}"
    fi
}

# Function to build the application
build_app() {
    echo -e "${BLUE}🔨 Building application...${NC}"
    
    cd "${APP_DIR}"
    
    if cargo build --release 2>"${LOG_DIR}/build.log"; then
        echo -e "${GREEN}✅ Build successful${NC}"
    else
        echo -e "${RED}❌ Build failed${NC}"
        echo -e "${RED}   Check logs: ${LOG_DIR}/build.log${NC}"
        tail -20 "${LOG_DIR}/build.log"
        exit 1
    fi
}

# Function to start the application
start_app() {
    echo -e "${BLUE}🚀 Starting ${APP_NAME}...${NC}"
    
    cd "${APP_DIR}"
    
    # Set environment variables
    export RUST_LOG=info
    export DATABASE_URL="sqlite:data/main.db"
    
    # Start the application in background
    nohup ./target/release/moodbridge_rust > "${LOG_DIR}/app.log" 2>&1 &
    echo $! > "${PID_FILE}"
    
    echo -e "${GREEN}✅ Application started${NC}"
    echo -e "${GREEN}   PID: $(cat ${PID_FILE})${NC}"
    echo -e "${GREEN}   Logs: ${LOG_DIR}/app.log${NC}"
    
    # Wait a moment and check if it's running
    sleep 2
    if ! kill -0 "$(cat ${PID_FILE})" 2>/dev/null; then
        echo -e "${RED}❌ Application failed to start${NC}"
        echo -e "${RED}   Check logs: ${LOG_DIR}/app.log${NC}"
        tail -20 "${LOG_DIR}/app.log"
        exit 1
    fi
    
    # Test API health
    echo -e "${BLUE}🔍 Testing API health...${NC}"
    for i in {1..10}; do
        if curl -s "http://localhost:${PORT}/api/health" >/dev/null 2>&1; then
            echo -e "${GREEN}✅ API is healthy${NC}"
            break
        fi
        
        if [ $i -eq 10 ]; then
            echo -e "${RED}❌ API health check failed${NC}"
            exit 1
        fi
        
        echo -e "${YELLOW}   Waiting for API... (${i}/10)${NC}"
        sleep 1
    done
}

# Function to open browser
open_browser() {
    echo -e "${BLUE}🌐 Opening browser...${NC}"
    
    url="http://localhost:${PORT}"
    
    if command -v open &> /dev/null; then
        # macOS
        open "${url}"
    elif command -v xdg-open &> /dev/null; then
        # Linux
        xdg-open "${url}"
    elif command -v start &> /dev/null; then
        # Windows
        start "${url}"
    else
        echo -e "${YELLOW}⚠️  Could not auto-open browser${NC}"
        echo -e "${GREEN}   Manually open: ${url}${NC}"
    fi
}

# Function to show status
show_status() {
    echo -e "${GREEN}🎉 ${APP_NAME} is now running!${NC}"
    echo -e "${GREEN}======================================${NC}"
    echo -e "${GREEN}🌐 Web Interface: http://localhost:${PORT}${NC}"
    echo -e "${GREEN}📊 Health Check:  http://localhost:${PORT}/api/health${NC}"
    echo -e "${GREEN}📁 Logs:          ${LOG_DIR}/${NC}"
    echo -e "${GREEN}🔧 Config:        data/main.db${NC}"
    echo ""
    echo -e "${BLUE}💡 Tips:${NC}"
    echo -e "   • Use Ctrl+C to stop the application"
    echo -e "   • Check logs if you encounter issues"
    echo -e "   • Data is stored locally in the data/ directory"
    echo ""
}

# Main execution
main() {
    # Change to app directory
    cd "${APP_DIR}"
    
    # Run all checks
    check_dependencies
    check_database
    
    # Check if already running
    if ! check_port; then
        exit 0
    fi
    
    # Build and start
    build_app
    start_app
    
    # Open browser and show status
    open_browser
    show_status
    
    # Keep running until interrupted
    echo -e "${BLUE}Press Ctrl+C to stop the application${NC}"
    trap "echo -e '\n${YELLOW}🛑 Stopping ${APP_NAME}...${NC}'; kill $(cat ${PID_FILE}) 2>/dev/null; rm -f ${PID_FILE}; echo -e '${GREEN}✅ Stopped${NC}'; exit 0" INT
    
    # Monitor the application
    while true; do
        if ! kill -0 "$(cat ${PID_FILE})" 2>/dev/null; then
            echo -e "${RED}❌ Application stopped unexpectedly${NC}"
            echo -e "${RED}   Check logs: ${LOG_DIR}/app.log${NC}"
            exit 1
        fi
        sleep 5
    done
}

# Handle command line arguments
case "${1:-}" in
    "stop")
        if [ -f "${PID_FILE}" ]; then
            echo -e "${YELLOW}🛑 Stopping ${APP_NAME}...${NC}"
            kill "$(cat ${PID_FILE})" 2>/dev/null && rm -f "${PID_FILE}"
            echo -e "${GREEN}✅ Stopped${NC}"
        else
            echo -e "${YELLOW}⚠️  Application is not running${NC}"
        fi
        exit 0
        ;;
    "status")
        if [ -f "${PID_FILE}" ] && kill -0 "$(cat ${PID_FILE})" 2>/dev/null; then
            echo -e "${GREEN}✅ ${APP_NAME} is running${NC}"
            echo -e "${GREEN}   PID: $(cat ${PID_FILE})${NC}"
            echo -e "${GREEN}   URL: http://localhost:${PORT}${NC}"
        else
            echo -e "${RED}❌ ${APP_NAME} is not running${NC}"
        fi
        exit 0
        ;;
    "restart")
        $0 stop
        sleep 2
        $0 start
        exit 0
        ;;
    "logs")
        if [ -f "${LOG_DIR}/app.log" ]; then
            tail -f "${LOG_DIR}/app.log"
        else
            echo -e "${RED}❌ No logs found${NC}"
        fi
        exit 0
        ;;
    "help"|"-h"|"--help")
        echo -e "${BLUE}${APP_NAME} Launcher${NC}"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  start    Start the application (default)"
        echo "  stop     Stop the application"
        echo "  restart  Restart the application"
        echo "  status   Check application status"
        echo "  logs     View application logs"
        echo "  help     Show this help message"
        exit 0
        ;;
    "start"|"")
        main
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo "Use '$0 help' for available commands"
        exit 1
        ;;
esac
