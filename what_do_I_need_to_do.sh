#!/bin/bash

# MoodBridge Rust - Quick Task Reminder
# Usage: ./what_do_I_need_to_do.sh [command]

PROJECT_DIR="/Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust"

echo "🚀 MoodBridge Rust - Task Reminder System"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$1" = "full" ]; then
    echo "📋 Opening full task list..."
    open "$PROJECT_DIR/TASK_MANAGER.md"
elif [ "$1" = "spec" ]; then
    echo "📋 Opening project specification..."
    open "$PROJECT_DIR/PROJECT_SPECIFICATION.md"
elif [ "$1" = "next" ]; then
    echo "⏭️  NEXT TASKS TO WORK ON:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "1. 🔴 [SEC-001] Implement OAuth2 client with authorization code flow"
    echo "   └─ Phase 1 | Security Infrastructure | Est: 16 hours"
    echo ""
    echo "2. 🟡 [ARCH-002] Complete PlatformIntegration trait implementation"
    echo "   └─ Phase 1 | Integration Framework | Est: 8 hours"
    echo ""
    echo "3. 🟢 [OBS-001] Configure structured logging with tracing crate"
    echo "   └─ Phase 1 | Observability Setup | Est: 6 hours"
    echo ""
    echo "Run './what_do_I_need_to_do.sh urgent' for critical tasks only"
elif [ "$1" = "urgent" ]; then
    echo "🚨 URGENT TASKS:"
    echo "━━━━━━━━━━━━━━━━━━"
    echo "🔴 [SEC-001] Implement OAuth2 client with authorization code flow"
    echo "   └─ Critical | 16 hours | Phase 1"
    echo ""
    echo "🟡 [ARCH-002] Complete PlatformIntegration trait implementation"
    echo "   └─ High | 8 hours | Phase 1"
elif [ "$1" = "status" ]; then
    echo "📊 PROJECT STATUS:"
    echo "━━━━━━━━━━━━━━━━━━━"
    echo "Total Tasks: 157"
    echo "Completed: 2 (1.3%)"
    echo "In Progress: 0"
    echo "Pending: 155"
    echo "Current Phase: Foundation & Architecture"
    echo "Next Milestone: OAuth2 Authentication Framework"
    echo "Est. Completion: October 2025"
    echo ""
    echo "⚠️  WARNING: Large project detected!"
    echo "   Focus on immediate Phase 1 tasks first."
elif [ "$1" = "run" ]; then
    echo "🔧 Running task manager..."
    cd "$PROJECT_DIR"
    cargo run --bin task_manager
else
    echo "📋 QUICK SUMMARY:"
    echo "━━━━━━━━━━━━━━━━━━"
    echo "Current Phase: Phase 1 - Foundation & Architecture"
    echo "Next Critical Task: Implement OAuth2 authentication"
    echo "Total Remaining: 155 tasks"
    echo ""
    echo "🎯 IMMEDIATE ACTIONS:"
    echo "━━━━━━━━━━━━━━━━━━━━"
    echo "1. Complete OAuth2 framework (SEC-001)"
    echo "2. Finish PlatformIntegration traits (ARCH-002)"
    echo "3. Set up logging infrastructure (OBS-001)"
    echo ""
    echo "📖 AVAILABLE COMMANDS:"
    echo "━━━━━━━━━━━━━━━━━━━━━"
    echo "./what_do_I_need_to_do.sh next    - Show next tasks"
    echo "./what_do_I_need_to_do.sh urgent  - Show urgent tasks only"
    echo "./what_do_I_need_to_do.sh status  - Show project status"
    echo "./what_do_I_need_to_do.sh full    - Open full task list"
    echo "./what_do_I_need_to_do.sh spec    - Open project spec"
    echo "./what_do_I_need_to_do.sh run     - Run Rust task manager"
    echo ""
    echo "⭐ TIP: Start with OAuth2 implementation (SEC-001)"
fi

echo ""
echo "📂 Project location: $PROJECT_DIR"
