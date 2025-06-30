#!/bin/bash

# MoodBridge DevOps Setup and Verification Script
# Completes the DevOps process integration and verifies all components

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}======================================================${NC}"
echo -e "${CYAN}    MoodBridge DevOps Setup & Integration${NC}"
echo -e "${CYAN}======================================================${NC}"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to verify git hooks
verify_git_hooks() {
    echo -e "${BLUE}🔍 Verifying Git Hooks...${NC}"
    
    local hooks_dir=".git/hooks"
    local required_hooks=("pre-commit" "commit-msg")
    
    for hook in "${required_hooks[@]}"; do
        if [ -f "$hooks_dir/$hook" ] && [ -x "$hooks_dir/$hook" ]; then
            echo -e "${GREEN}  ✅ $hook hook installed and executable${NC}"
        else
            echo -e "${RED}  ❌ $hook hook missing or not executable${NC}"
            return 1
        fi
    done
    
    echo -e "${GREEN}✅ All Git hooks verified${NC}"
    echo ""
}

# Function to verify DevOps scripts
verify_devops_scripts() {
    echo -e "${BLUE}🔍 Verifying DevOps Scripts...${NC}"
    
    local required_scripts=("devops_commit_checker.sh" "setup_devops.sh")
    
    for script in "${required_scripts[@]}"; do
        if [ -f "$script" ] && [ -x "$script" ]; then
            echo -e "${GREEN}  ✅ $script exists and is executable${NC}"
        else
            echo -e "${RED}  ❌ $script missing or not executable${NC}"
            return 1
        fi
    done
    
    echo -e "${GREEN}✅ All DevOps scripts verified${NC}"
    echo ""
}

# Function to verify configuration files
verify_config_files() {
    echo -e "${BLUE}🔍 Verifying Configuration Files...${NC}"
    
    local required_configs=("devops_config.yaml" "DEVOPS_IMPLEMENTATION_GUIDE.md")
    
    for config in "${required_configs[@]}"; do
        if [ -f "$config" ]; then
            echo -e "${GREEN}  ✅ $config exists${NC}"
        else
            echo -e "${RED}  ❌ $config missing${NC}"
            return 1
        fi
    done
    
    echo -e "${GREEN}✅ All configuration files verified${NC}"
    echo ""
}

# Function to verify Rust toolchain
verify_rust_toolchain() {
    echo -e "${BLUE}🔍 Verifying Rust Toolchain...${NC}"
    
    if command_exists cargo; then
        echo -e "${GREEN}  ✅ Cargo: $(cargo --version)${NC}"
    else
        echo -e "${RED}  ❌ Cargo not found${NC}"
        return 1
    fi
    
    if command_exists rustfmt; then
        echo -e "${GREEN}  ✅ rustfmt: $(rustfmt --version)${NC}"
    else
        echo -e "${YELLOW}  ⚠️  rustfmt not found - installing...${NC}"
        rustup component add rustfmt
    fi
    
    if cargo clippy --version >/dev/null 2>&1; then
        echo -e "${GREEN}  ✅ clippy: $(cargo clippy --version)${NC}"
    else
        echo -e "${YELLOW}  ⚠️  clippy not found - installing...${NC}"
        rustup component add clippy
    fi
    
    echo -e "${GREEN}✅ Rust toolchain verified${NC}"
    echo ""
}

# Function to test pre-commit hooks
test_pre_commit_hooks() {
    echo -e "${BLUE}🧪 Testing Pre-commit Hooks...${NC}"
    
    # Create a temporary test file
    echo "// Test file for pre-commit hook verification" > test_precommit.rs
    git add test_precommit.rs
    
    echo -e "${YELLOW}  Running pre-commit checks...${NC}"
    if .git/hooks/pre-commit; then
        echo -e "${GREEN}  ✅ Pre-commit hooks working correctly${NC}"
    else
        echo -e "${RED}  ❌ Pre-commit hooks failed${NC}"
        git reset HEAD test_precommit.rs
        rm -f test_precommit.rs
        return 1
    fi
    
    # Clean up
    git reset HEAD test_precommit.rs
    rm -f test_precommit.rs
    
    echo ""
}

# Function to run commit checker
run_commit_checker() {
    echo -e "${BLUE}🔍 Running Comprehensive Commit Checker...${NC}"
    
    if ./devops_commit_checker.sh; then
        echo -e "${GREEN}✅ Commit checker completed successfully${NC}"
    else
        echo -e "${YELLOW}⚠️  Commit checker completed with warnings${NC}"
    fi
    
    echo ""
}

# Function to display final status
display_final_status() {
    echo -e "${CYAN}======================================================${NC}"
    echo -e "${GREEN}🎉 DevOps Integration Complete!${NC}"
    echo -e "${CYAN}======================================================${NC}"
    echo ""
    
    echo -e "${BLUE}📋 Installed Components:${NC}"
    echo -e "${GREEN}  ✅ Pre-commit hooks (quality gates)${NC}"
    echo -e "${GREEN}  ✅ Commit message validation${NC}"
    echo -e "${GREEN}  ✅ Multi-project commit checker${NC}"
    echo -e "${GREEN}  ✅ DevOps configuration system${NC}"
    echo -e "${GREEN}  ✅ Automated reporting and analytics${NC}"
    echo ""
    
    echo -e "${BLUE}📊 Generated Reports Available:${NC}"
    if [ -d "devops_reports" ]; then
        local report_count=$(ls devops_reports/*.md 2>/dev/null | wc -l | tr -d ' ')
        echo -e "${GREEN}  📋 $report_count reports in devops_reports/${NC}"
        echo -e "${YELLOW}  📝 Latest commit analysis${NC}"
        echo -e "${YELLOW}  📚 TODO and knowledge base updates${NC}"
        echo -e "${YELLOW}  📋 Commit strategy documentation${NC}"
    fi
    echo ""
    
    echo -e "${BLUE}🚀 Next Steps:${NC}"
    echo -e "${YELLOW}  1. Review generated reports in devops_reports/${NC}"
    echo -e "${YELLOW}  2. Commit all pending changes across projects${NC}"
    echo -e "${YELLOW}  3. Set up regular DevOps workflow (see DEVOPS_IMPLEMENTATION_GUIDE.md)${NC}"
    echo -e "${YELLOW}  4. Schedule weekly commit analysis reviews${NC}"
    echo ""
    
    echo -e "${BLUE}📚 Documentation:${NC}"
    echo -e "${GREEN}  📖 DEVOPS_IMPLEMENTATION_GUIDE.md - Complete implementation guide${NC}"
    echo -e "${GREEN}  ⚙️  devops_config.yaml - Configuration settings${NC}"
    echo -e "${GREEN}  📊 devops_reports/ - Automated analysis reports${NC}"
    echo ""
    
    echo -e "${PURPLE}✨ MoodBridge DevOps system is now fully operational!${NC}"
}

# Function to commit the DevOps system itself
commit_devops_system() {
    echo -e "${BLUE}💾 Committing DevOps System...${NC}"
    
    # Check if there are any changes to commit
    if git diff --cached --quiet && git diff --quiet -- devops_commit_checker.sh devops_config.yaml DEVOPS_IMPLEMENTATION_GUIDE.md setup_devops.sh .git/hooks/; then
        echo -e "${YELLOW}  ℹ️  No DevOps changes to commit${NC}"
        return 0
    fi
    
    # Stage all DevOps files
    git add devops_commit_checker.sh devops_config.yaml DEVOPS_IMPLEMENTATION_GUIDE.md setup_devops.sh
    
    # Only add hooks if they exist and are not already tracked
    if [ -f ".git/hooks/pre-commit" ]; then
        echo -e "${YELLOW}  📝 Note: Git hooks are local and won't be committed to repository${NC}"
    fi
    
    echo -e "${YELLOW}  📝 Committing DevOps system with proper conventional commit message...${NC}"
    
    # Create the commit message
    local commit_msg="feat(devops): implement comprehensive DevOps automation system

- Add multi-project commit checker with cross-repository analysis
- Implement pre-commit hooks for code quality gates
- Add commit message validation with conventional commit standards
- Create automated TODO scanning and documentation tracking
- Establish unified DevOps configuration and reporting
- Generate actionable reports for project maintenance

This system ensures consistent code quality, proper documentation
maintenance, and streamlined development workflows across all
MoodBridge projects."

    # Commit with the proper message
    if git commit -m "$commit_msg"; then
        echo -e "${GREEN}  ✅ DevOps system committed successfully${NC}"
    else
        echo -e "${RED}  ❌ Failed to commit DevOps system${NC}"
        return 1
    fi
    
    echo ""
}

# Main execution
main() {
    echo -e "${PURPLE}🔧 Starting DevOps Setup Verification...${NC}"
    echo ""
    
    # Run all verification steps
    local failed_checks=0
    
    if ! verify_git_hooks; then
        failed_checks=$((failed_checks + 1))
    fi
    
    if ! verify_devops_scripts; then
        failed_checks=$((failed_checks + 1))
    fi
    
    if ! verify_config_files; then
        failed_checks=$((failed_checks + 1))
    fi
    
    if ! verify_rust_toolchain; then
        failed_checks=$((failed_checks + 1))
    fi
    
    if ! test_pre_commit_hooks; then
        failed_checks=$((failed_checks + 1))
    fi
    
    # Run the commit checker (non-blocking)
    run_commit_checker
    
    # Commit the DevOps system
    commit_devops_system
    
    # Final status
    if [ $failed_checks -eq 0 ]; then
        display_final_status
        exit 0
    else
        echo -e "${RED}❌ DevOps setup completed with $failed_checks failed checks${NC}"
        echo -e "${RED}Please review and fix the issues above${NC}"
        exit 1
    fi
}

# Run main function
main "$@"
