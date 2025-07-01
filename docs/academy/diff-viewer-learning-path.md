# MoodBridge Diff Viewer Academy Learning Path

## Overview

This learning path guides you through understanding and using the MoodBridge Diff Viewer system, a powerful tool for comparing legal documents and managing version control in legal workflows.

## Learning Objectives

By the end of this path, you will be able to:
- Understand the architecture of the MoodBridge diff viewer
- Compare legal documents efficiently
- Use the commit functionality for version control
- Integrate the diff viewer into legal workflows
- Troubleshoot common issues

## Prerequisites

- Basic understanding of Git version control
- Familiarity with Markdown syntax
- Basic knowledge of web applications
- Understanding of legal document workflows

## Module 1: Architecture Overview (15 minutes)

### 1.1 System Components

The MoodBridge diff viewer consists of:

1. **Backend API** (`src/handlers/mod.rs`)
   - `diff_viewer()` - Serves the HTML interface
   - `diff_data()` - Provides file comparison data
   - `commit_changes()` - Handles file saves and git commits

2. **Frontend Interface** (`templates/diff_viewer.html`)
   - Monaco Editor integration for syntax highlighting
   - Side-by-side diff visualization
   - Commit controls

3. **Git Integration**
   - Automatic staging and committing
   - SHA-256 hash verification
   - Chain of custody maintenance

### 1.2 Data Flow

```
User Request → Axum Router → Handler → File System → Git → Response
```

## Module 2: Using the Diff Viewer (20 minutes)

### 2.1 Accessing the Interface

1. Start MoodBridge server: `cargo run`
2. Navigate to: `http://localhost:8080/diff`
3. The interface loads with default file paths

### 2.2 Comparing Files

The system defaults to comparing:
- File 1: `/Users/tyler/Documents/mallory-legal-20250629/LegalReviewofVersion5.md`
- File 2: `/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 5.md`

#### API Usage:
```bash
curl "http://localhost:8080/api/diff-data?file1=/path/to/file1&file2=/path/to/file2"
```

### 2.3 Understanding Diff Output

The API returns structured diff data:
```json
{
  "file1": {
    "path": "/path/to/file1",
    "content": "...",
    "lines": 100
  },
  "file2": {
    "path": "/path/to/file2", 
    "content": "...",
    "lines": 105
  },
  "diff": [
    {
      "type": "addition",
      "line_number": 50,
      "content": "New line added"
    }
  ]
}
```

## Module 3: Commit Workflow (25 minutes)

### 3.1 Making Changes

1. Edit content in the right panel (Monaco editor)
2. Click "Commit Changes" when ready
3. System automatically:
   - Saves to target file
   - Stages changes in git
   - Creates commit with descriptive message

### 3.2 API Commit Process

```bash
curl -X POST http://localhost:8080/api/commit-changes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Updated document content...",
    "target_file": "/path/to/output/file.md"
  }'
```

### 3.3 Chain of Custody

All files maintain:
- SHA-256 hashing for integrity
- Git history for auditability  
- Timestamp tracking
- Legal document standards compliance

## Module 4: Integration Patterns (30 minutes)

### 4.1 Legal Workflow Integration

Typical legal document workflow:
1. **Review Phase**: Compare draft vs. reviewed version
2. **Edit Phase**: Make corrections in diff viewer
3. **Approval Phase**: Commit final version
4. **Filing Phase**: Export to required formats

### 4.2 Version Control Best Practices

- Use descriptive commit messages
- Tag important versions (e.g., "v6-final-for-filing")
- Maintain branch separation for different cases
- Regular backups of git repository

### 4.3 Security Considerations

- File paths are validated server-side
- Git operations are isolated to specific directories
- SHA-256 verification prevents tampering
- Access control through web server configuration

## Module 5: Advanced Features (20 minutes)

### 5.1 Custom File Paths

Modify default paths by updating handler configuration:

```rust
let file1_path = params
    .file1
    .as_deref()
    .unwrap_or("/custom/default/path1.md");
```

### 5.2 Extending Diff Algorithms

Current implementation uses line-by-line comparison. Can be extended for:
- Word-level diffs
- Semantic comparison
- Legal citation tracking
- Cross-reference validation

### 5.3 Integration with External Tools

- Export diffs to PDF for court filing
- Integration with legal databases
- Automated citation checking
- Template-based document generation

## Module 6: Troubleshooting (15 minutes)

### 6.1 Common Issues

**File Not Found Errors:**
- Verify file paths exist
- Check file permissions
- Ensure proper escaping of paths with spaces

**Git Commit Failures:**
- Verify git repository initialization
- Check write permissions
- Ensure git user configuration

**Monaco Editor Issues:**
- Verify CDN accessibility
- Check browser JavaScript console
- Ensure proper Content-Type headers

### 6.2 Debugging Tools

```bash
# Check server logs
tail -f logs/moodbridge.log

# Verify git status
git status /path/to/legal/docs

# Test API endpoints
curl -v http://localhost:8080/api/health
```

## Module 7: Practical Exercise (45 minutes)

### Exercise: Complete Document Review Workflow

1. **Setup Phase** (10 minutes)
   - Start MoodBridge server
   - Prepare two test documents
   - Verify git repository status

2. **Comparison Phase** (15 minutes)
   - Load documents in diff viewer
   - Identify key differences
   - Document findings

3. **Editing Phase** (15 minutes)
   - Make corrections based on review
   - Test live preview functionality
   - Validate changes

4. **Commit Phase** (5 minutes)
   - Commit changes with descriptive message
   - Verify git history
   - Check file integrity hashes

## Assessment Checklist

- [ ] Can navigate the diff viewer interface
- [ ] Successfully compare two documents
- [ ] Understand diff output format
- [ ] Complete commit workflow
- [ ] Troubleshoot common issues
- [ ] Explain security features
- [ ] Demonstrate legal workflow integration

## Next Steps

After completing this learning path:
1. Explore advanced customization options
2. Review integration with other MoodBridge modules
3. Study legal document templates
4. Practice with real case documents

## Resources

- [MoodBridge Architecture Guide](../architecture/overview.md)
- [API Documentation](../api/endpoints.md)
- [Legal Workflow Guide](../workflows/legal-process.md)
- [Security Guidelines](../security/best-practices.md)

## Support

For questions or issues:
- Check the troubleshooting guide
- Review GitHub issues
- Contact the development team
- Consult legal workflow documentation
