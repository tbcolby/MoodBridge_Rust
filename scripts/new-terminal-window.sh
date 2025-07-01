#!/bin/bash

# Robust Terminal Window Setup for MoodBridge_Rust
# Usage: ./scripts/new-terminal-window.sh [feature-name]

PROJECT_ROOT="/Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust"
USER_PREFIX="tyler"

# Function to display usage
usage() {
    echo "Usage: $0 [feature-name]"
    echo "Example: $0 browser-integration"
    echo "Example: $0 fabric-integration-fix"
    exit 1
}

# Function to setup new terminal window
setup_new_terminal() {
    local feature_name="$1"
    local branch_name="${USER_PREFIX}/${feature_name}"
    
    echo "🚀 Setting up new terminal window for: $feature_name"
    echo "📁 Project: MoodBridge_Rust"
    echo "🌿 Branch: $branch_name"
    echo ""
    
    # Step 1: Navigate to project root
    cd "$PROJECT_ROOT" || {
        echo "❌ Error: Could not navigate to project root"
        exit 1
    }
    
    # Step 2: Ensure main is clean and up-to-date
    echo "📥 Updating main branch..."
    git checkout main
    git pull origin main
    
    # Step 3: Create and switch to feature branch
    echo "🌿 Creating feature branch: $branch_name"
    git checkout -b "$branch_name"
    
    # Step 4: Set terminal title
    echo -ne "\033]0;MoodBridge - $branch_name\007"
    
    # Step 5: Display current status
    echo ""
    echo "✅ Terminal window setup complete!"
    echo "📍 Current branch: $(git branch --show-current)"
    echo "📂 Working directory: $(pwd)"
    echo ""
    echo "🎯 You are now ready to work on: $feature_name"
    echo "⚠️  Remember: Only work on this feature in this terminal window"
    echo ""
    
    # Display helpful commands
    echo "📋 Helpful commands:"
    echo "  git status                    # Check current status"
    echo "  git add . && git commit -m \"\" # Commit work"
    echo "  git push -u origin $branch_name # Push to remote"
    echo ""
}

# Main script logic
if [ $# -eq 0 ]; then
    echo "⚠️  No feature name provided"
    echo ""
    usage
fi

feature_name="$1"

# Validate feature name (no spaces, special chars)
if [[ ! "$feature_name" =~ ^[a-zA-Z0-9-_]+$ ]]; then
    echo "❌ Error: Feature name should only contain letters, numbers, hyphens, and underscores"
    exit 1
fi

setup_new_terminal "$feature_name"
