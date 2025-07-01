# Robust Terminal Window Procedure for MoodBridge_Rust

## Overview
This procedure prevents merge conflicts by ensuring each terminal window works on a dedicated git branch, eliminating the mixing of different threads/features.

## Core Principle
**One Terminal Window = One Feature Branch = One Thread of Work**

## Quick Start

### For New Features
```bash
# Run this in any new terminal window
./scripts/new-terminal-window.sh your-feature-name
```

Example:
```bash
./scripts/new-terminal-window.sh browser-integration
./scripts/new-terminal-window.sh fabric-fix
./scripts/new-terminal-window.sh thread-collapse
```

## Manual Setup (if script not available)

### 1. Navigate to Project
```bash
cd "/Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust"
```

### 2. Update Main Branch
```bash
git checkout main
git pull origin main
```

### 3. Create Feature Branch
```bash
git checkout -b tyler/your-feature-name
```

### 4. Set Terminal Title (Optional)
```bash
echo -ne "\033]0;MoodBridge - $(git branch --show-current)\007"
```

## Daily Workflow

### Starting Work
1. Open new terminal window/tab
2. Run: `./scripts/new-terminal-window.sh feature-name`
3. Work exclusively in this terminal for this feature

### During Work
- Use `git status` frequently to check your changes
- Commit regularly: `git add . && git commit -m "Description"`
- Push to remote: `git push -u origin $(git branch --show-current)`

### Switching Between Features
- **Never** change branches in an active terminal
- Use different terminal windows for different features
- Each window maintains its own branch context

### Ending Work Session
```bash
# Always commit before closing
git add .
git commit -m "WIP: Save progress on [feature]"
git push origin $(git branch --show-current)
```

## Branch Management

### Branch Naming Convention
- Format: `tyler/descriptive-feature-name`
- Examples:
  - `tyler/browser-integration`
  - `tyler/fix-fabric-connection`
  - `tyler/thread-collapse-system`
  - `tyler/legal-analysis-engine`

### Integration Strategy
When ready to merge features back to main:

```bash
# In your feature branch terminal
git checkout main
git pull origin main
git checkout your-feature-branch
git rebase main
# Resolve any conflicts
git push --force-with-lease origin your-feature-branch

# Then create PR/merge via GitHub
```

## Emergency Procedures

### Mixed Work in Wrong Branch
```bash
# Save current changes
git stash push -m "Mixed work - needs proper branch"

# Create correct branch
git checkout main
git checkout -b tyler/correct-feature-name

# Restore work
git stash pop
```

### Accidental Branch Switch
```bash
# If you accidentally switched branches with uncommitted changes
git checkout - # Go back to previous branch
git add .
git commit -m "WIP: Saved work"
```

### Lost Terminal Context
```bash
# Check what branch you're on
git branch --show-current

# Check what you were working on
git log --oneline -5

# Set terminal title to match
echo -ne "\033]0;MoodBridge - $(git branch --show-current)\007"
```

## Current Saved Work

Your previous mixed work has been saved in branch: `tyler/temp-current-work`

To split this into proper feature branches:
1. Check what's in the temp branch: `git checkout tyler/temp-current-work && git log --oneline`
2. Create proper feature branches for each logical group of changes
3. Cherry-pick or move relevant commits to each feature branch

## Benefits of This Procedure

✅ **No More Merge Conflicts**: Each terminal works in isolation
✅ **Clear Context**: Terminal title shows current feature
✅ **Easy Switching**: Different windows for different features  
✅ **Safe Experimentation**: Each feature is isolated
✅ **Clean History**: Logical commits per feature
✅ **Parallel Development**: Work on multiple features simultaneously

## Troubleshooting

### "Branch already exists"
```bash
# If branch exists, either switch to it or create with different name
git checkout tyler/existing-branch-name
# OR
git checkout -b tyler/feature-name-v2
```

### Script Permission Denied
```bash
chmod +x scripts/new-terminal-window.sh
```

### Can't Find Script
```bash
# Make sure you're in project root
pwd
# Should show: /Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust
```

---

**Remember**: The key to avoiding merge conflicts is discipline in keeping each terminal window focused on one feature branch. Never mix work from different threads in the same terminal!
