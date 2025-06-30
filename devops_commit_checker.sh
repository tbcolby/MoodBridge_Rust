#!/bin/bash

# MoodBridge DevOps Commit Checker
# Comprehensive process to check all commits across all projects
# Created: $(date)
# Author: DevOps Automation System

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPORT_DIR="${SCRIPT_DIR}/devops_reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="${REPORT_DIR}/commit_check_${TIMESTAMP}.md"
TODO_FILE="${REPORT_DIR}/todos_${TIMESTAMP}.md"
KNOWLEDGE_BASE_FILE="${REPORT_DIR}/knowledge_base_update_${TIMESTAMP}.md"

# Project directories to scan
declare -a PROJECT_DIRS=(
    "/private/tmp/MoodBridge_Rust"
    "/Users/tyler/Library/Mobile Documents/com~apple~CloudDocs/Legal/MoodBridge_Rust"
    "/Users/tyler/Desktop/DevCopilot-Pro"
    "/Users/tyler/Desktop/JPMC_Zero_Trust"
    "/Users/tyler/Desktop"
    "/Users/tyler/Desktop/MoodBridge-LangChain-API"
    "/Users/tyler/Desktop/Zero_Trust_Org_Sync"
)

# Initialize report directory
mkdir -p "$REPORT_DIR"

echo -e "${CYAN}=====================================================${NC}"
echo -e "${CYAN}    MoodBridge DevOps Commit Checker v1.0${NC}"
echo -e "${CYAN}=====================================================${NC}"
echo -e "${BLUE}Starting comprehensive commit analysis...${NC}"
echo -e "${BLUE}Report will be saved to: $REPORT_FILE${NC}"
echo ""

# Initialize report files
touch "$REPORT_FILE" "$TODO_FILE" "$KNOWLEDGE_BASE_FILE"

cat > "$REPORT_FILE" << EOF
# DevOps Commit Analysis Report
Generated: $(date)
Analysis Type: Comprehensive Multi-Project Commit Check

## Executive Summary
This report provides a comprehensive analysis of all commits across MoodBridge and related projects.

## Projects Analyzed
EOF

cat > "$TODO_FILE" << EOF
# Generated TODOs from Commit Analysis
Generated: $(date)

## Critical TODOs
EOF

cat > "$KNOWLEDGE_BASE_FILE" << EOF
# Knowledge Base Updates
Generated: $(date)

## Technical Documentation Updates Required
EOF

# Function to analyze a git repository
analyze_repo() {
    local repo_path="$1"
    local repo_name=$(basename "$repo_path")
    local original_dir=$(pwd)
    
    echo -e "${YELLOW}Analyzing repository: $repo_name${NC}"
    echo -e "${YELLOW}Path: $repo_path${NC}"
    
    if [ ! -d "$repo_path/.git" ]; then
        echo -e "${RED}❌ Not a git repository: $repo_path${NC}"
        echo "- $repo_name: ❌ Not a git repository" >> "$REPORT_FILE"
        return 1
    fi
    
    cd "$repo_path"
    
    # Get repository status
    local status_output=$(git status --porcelain 2>/dev/null || echo "ERROR")
    local branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    local last_commit=$(git --no-pager log -1 --format="%h %s" 2>/dev/null || echo "No commits")
    local uncommitted_changes=$(echo "$status_output" | wc -l | tr -d ' ')
    
    # Check for unpushed commits
    local unpushed=""
    if git remote -v | grep -q origin; then
        local ahead_behind=$(git rev-list --left-right --count origin/$branch...$branch 2>/dev/null || echo "0	0")
        local ahead=$(echo "$ahead_behind" | cut -f1)
        local behind=$(echo "$ahead_behind" | cut -f2)
        if [ "$ahead" -gt 0 ]; then
            unpushed="$ahead commits ahead of origin"
        fi
    fi
    
    # Add to report
    echo "" >> "$REPORT_FILE"
    echo "### $repo_name" >> "$REPORT_FILE"
    echo "- **Path**: \`$repo_path\`" >> "$REPORT_FILE"
    echo "- **Branch**: $branch" >> "$REPORT_FILE"
    echo "- **Last Commit**: $last_commit" >> "$REPORT_FILE"
    echo "- **Uncommitted Changes**: $uncommitted_changes files" >> "$REPORT_FILE"
    echo "- **Status**: $unpushed" >> "$REPORT_FILE"
    
    # Check for specific patterns that need attention
    check_for_todos "$repo_path" "$repo_name"
    check_for_documentation_updates "$repo_path" "$repo_name"
    
    # Display status
    if [ "$uncommitted_changes" -gt 0 ]; then
        echo -e "${RED}  ⚠️  Uncommitted changes: $uncommitted_changes files${NC}"
        echo "  📋 Files:"
        git status --porcelain | head -10 | while read line; do
            echo -e "${YELLOW}      $line${NC}"
        done
    fi
    
    if [ -n "$unpushed" ]; then
        echo -e "${YELLOW}  📤 $unpushed${NC}"
    fi
    
    echo -e "${GREEN}  ✅ Last commit: $last_commit${NC}"
    echo ""
}

# Function to check for TODOs in code
check_for_todos() {
    local repo_path="$1"
    local repo_name="$2"
    
    local todo_count=$(find "$repo_path" -type f \( -name "*.rs" -o -name "*.py" -o -name "*.js" -o -name "*.md" -o -name "*.sh" \) -exec grep -l "TODO\|FIXME\|HACK\|XXX" {} \; 2>/dev/null | wc -l | tr -d ' ')
    
    if [ "$todo_count" -gt 0 ]; then
        echo "" >> "$TODO_FILE"
        echo "### $repo_name TODOs" >> "$TODO_FILE"
        find "$repo_path" -type f \( -name "*.rs" -o -name "*.py" -o -name "*.js" -o -name "*.md" -o -name "*.sh" \) -exec grep -Hn "TODO\|FIXME\|HACK\|XXX" {} \; 2>/dev/null | head -20 >> "$TODO_FILE"
    fi
}

# Function to check for documentation that needs updating
check_for_documentation_updates() {
    local repo_path="$1"
    local repo_name="$2"
    
    # Check for README files that might need updating
    local readme_files=$(find "$repo_path" -name "README*" -o -name "*.md" | head -10)
    
    if [ -n "$readme_files" ]; then
        echo "" >> "$KNOWLEDGE_BASE_FILE"
        echo "### $repo_name Documentation Files" >> "$KNOWLEDGE_BASE_FILE"
        echo "$readme_files" | while read file; do
            if [ -f "$file" ]; then
                local last_modified=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "$file" 2>/dev/null || echo "unknown")
                echo "- \`$(basename "$file")\`: Last modified $last_modified" >> "$KNOWLEDGE_BASE_FILE"
            fi
        done
    fi
}

# Function to generate summary recommendations
generate_recommendations() {
    echo "" >> "$REPORT_FILE"
    echo "## Recommendations" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "### Immediate Actions Required" >> "$REPORT_FILE"
    echo "1. **Commit Uncommitted Changes**: Review and commit all pending changes" >> "$REPORT_FILE"
    echo "2. **Push Pending Commits**: Ensure all local commits are pushed to remote repositories" >> "$REPORT_FILE"
    echo "3. **Review TODOs**: Address critical TODOs identified in the codebase" >> "$REPORT_FILE"
    echo "4. **Update Documentation**: Ensure all documentation is current and accurate" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "### DevOps Process Improvements" >> "$REPORT_FILE"
    echo "1. **Automated Commit Checking**: Implement pre-commit hooks" >> "$REPORT_FILE"
    echo "2. **Continuous Integration**: Set up CI/CD pipelines for all projects" >> "$REPORT_FILE"
    echo "3. **Documentation Automation**: Implement automatic documentation generation" >> "$REPORT_FILE"
    echo "4. **Code Quality Gates**: Implement automated code quality checks" >> "$REPORT_FILE"
}

# Function to create unified commit strategy
create_commit_strategy() {
    local strategy_file="${REPORT_DIR}/commit_strategy_${TIMESTAMP}.md"
    
    cat > "$strategy_file" << EOF
# Unified Commit Strategy
Generated: $(date)

## Commit Message Standards
- Use conventional commits format: \`type(scope): description\`
- Types: feat, fix, docs, style, refactor, test, chore
- Include issue references where applicable

## Branch Strategy
- main: Production-ready code
- develop: Integration branch for features
- feature/*: Feature development branches
- hotfix/*: Critical bug fixes

## Review Process
1. Code review required for all commits to main
2. Automated testing must pass
3. Documentation must be updated
4. Security scan must pass

## Automation Recommendations
1. Pre-commit hooks for code formatting
2. Automated testing on push
3. Dependency vulnerability scanning
4. Documentation generation
EOF

    echo -e "${GREEN}✅ Commit strategy created: $strategy_file${NC}"
}

# Main execution
echo -e "${PURPLE}Scanning project directories...${NC}"

for project_dir in "${PROJECT_DIRS[@]}"; do
    if [ -d "$project_dir" ]; then
        analyze_repo "$project_dir"
    else
        echo -e "${RED}❌ Directory not found: $project_dir${NC}"
        echo "- $(basename "$project_dir"): ❌ Directory not found" >> "$REPORT_FILE"
    fi
done

# Generate recommendations and strategy
generate_recommendations
create_commit_strategy

# Final summary
echo -e "${CYAN}=====================================================${NC}"
echo -e "${GREEN}✅ DevOps Commit Analysis Complete!${NC}"
echo -e "${CYAN}=====================================================${NC}"
echo ""
echo -e "${BLUE}📊 Reports Generated:${NC}"
echo -e "${YELLOW}   📋 Main Report: $REPORT_FILE${NC}"
echo -e "${YELLOW}   📝 TODOs: $TODO_FILE${NC}"
echo -e "${YELLOW}   📚 Knowledge Base: $KNOWLEDGE_BASE_FILE${NC}"
echo -e "${YELLOW}   📋 Commit Strategy: ${REPORT_DIR}/commit_strategy_${TIMESTAMP}.md${NC}"
echo ""
echo -e "${BLUE}📋 Next Steps:${NC}"
echo -e "${GREEN}   1. Review the generated reports${NC}"
echo -e "${GREEN}   2. Address any uncommitted changes${NC}"
echo -e "${GREEN}   3. Push pending commits${NC}"
echo -e "${GREEN}   4. Review and resolve TODOs${NC}"
echo -e "${GREEN}   5. Update documentation as needed${NC}"
echo ""
echo -e "${PURPLE}🚀 DevOps Process Integration Complete!${NC}"
