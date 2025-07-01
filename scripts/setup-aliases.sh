#!/bin/bash

# Setup aliases for MoodBridge terminal window procedure
# Run this once: source scripts/setup-aliases.sh

PROJECT_ROOT="/Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust"

# Alias for creating new terminal windows
alias new-window="$PROJECT_ROOT/scripts/new-terminal-window.sh"
alias nw="$PROJECT_ROOT/scripts/new-terminal-window.sh"

# Alias for quick navigation to project
alias moodbridge="cd '$PROJECT_ROOT'"
alias mb="cd '$PROJECT_ROOT'"

# Git aliases for common operations
alias gst="git status"
alias gco="git checkout"
alias gaa="git add ."
alias gcm="git commit -m"
alias gps="git push origin \$(git branch --show-current)"
alias gpl="git pull origin main"

echo "✅ MoodBridge aliases loaded!"
echo ""
echo "Available commands:"
echo "  new-window [feature-name]  # Create new terminal window with feature branch"
echo "  nw [feature-name]          # Short version of above"
echo "  moodbridge                 # Navigate to project root"
echo "  mb                         # Short version of above"
echo ""
echo "Git shortcuts:"
echo "  gst     # git status"
echo "  gaa     # git add ."
echo "  gcm     # git commit -m"
echo "  gps     # git push origin current-branch"
echo ""
echo "Usage example:"
echo "  nw browser-integration     # Sets up new window for browser work"
