# Diff Viewer API Documentation

## Overview

The MoodBridge Diff Viewer API provides endpoints for comparing files, visualizing differences, and committing changes with git integration.

## Base URL

```
http://localhost:8080
```

## Authentication

Currently, no authentication is required. In production, implement appropriate access controls.

## Endpoints

### GET /diff

Serves the diff viewer HTML interface.

**Response:**
- Content-Type: `text/html`
- Status: `200 OK`

**Example:**
```bash
curl http://localhost:8080/diff
```

### GET /api/diff-data

Compares two files and returns structured diff data.

**Query Parameters:**
- `file1` (optional): Path to first file for comparison
- `file2` (optional): Path to second file for comparison

**Default Values:**
- `file1`: `/Users/tyler/Documents/mallory-legal-20250629/LegalReviewofVersion5.md`
- `file2`: `/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 5.md`

**Response:**
```json
{
  "file1": {
    "path": "string",
    "content": "string",
    "lines": "integer"
  },
  "file2": {
    "path": "string", 
    "content": "string",
    "lines": "integer"
  },
  "diff": [
    {
      "type": "string",
      "line_number": "integer",
      "content": "string",
      "old_content": "string",
      "new_content": "string"
    }
  ],
  "timestamp": "string (ISO 8601)"
}
```

**Diff Types:**
- `unchanged`: Line exists in both files with same content
- `addition`: Line exists only in file2
- `deletion`: Line exists only in file1  
- `modification`: Line exists in both files but content differs

**Example:**
```bash
curl "http://localhost:8080/api/diff-data?file1=/path/to/file1.md&file2=/path/to/file2.md"
```

**Status Codes:**
- `200 OK`: Successful comparison
- `404 NOT_FOUND`: One or both files not found
- `500 INTERNAL_SERVER_ERROR`: Server error during comparison

### POST /api/commit-changes

Saves content to a file and commits changes to git repository.

**Request Body:**
```json
{
  "content": "string (required)",
  "target_file": "string (optional)"
}
```

**Default Values:**
- `target_file`: `/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 6.md`

**Response:**
```json
{
  "success": "boolean",
  "message": "string",
  "file_path": "string",
  "git_committed": "boolean",
  "timestamp": "string (ISO 8601)"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/commit-changes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "# Updated Document\n\nThis is the new content...",
    "target_file": "/path/to/output.md"
  }'
```

**Status Codes:**
- `200 OK`: File saved successfully
- `400 BAD_REQUEST`: Invalid request body
- `500 INTERNAL_SERVER_ERROR`: File write or git operation failed

## Error Handling

All endpoints return structured error responses:

```json
{
  "error": "string",
  "message": "string",
  "timestamp": "string (ISO 8601)"
}
```

## Git Integration

The commit endpoint automatically:

1. **Saves File**: Writes content to specified path
2. **Git Add**: Stages the file for commit
3. **Git Commit**: Creates commit with descriptive message
4. **Logging**: Records operation details

**Git Commit Message Format:**
```
Update {filename}
```

**Git Repository Requirements:**
- Target directory must be git-initialized
- Write permissions required
- Git user configuration recommended

## File Path Security

- Paths are validated server-side
- Directory traversal attacks prevented
- Only specified directories accessible
- File permissions respected

## Rate Limiting

Currently no rate limiting implemented. Consider adding for production:
- Per-IP request limits
- File operation throttling
- Git commit frequency limits

## Examples

### Complete Workflow Example

1. **Get Diff Data:**
```bash
curl "http://localhost:8080/api/diff-data" | jq .
```

2. **Commit Changes:**
```bash
curl -X POST http://localhost:8080/api/commit-changes \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Updated legal document content...",
    "target_file": "/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 6.md"
  }' | jq .
```

### JavaScript Integration

```javascript
// Fetch diff data
const diffResponse = await fetch('/api/diff-data');
const diffData = await diffResponse.json();

// Commit changes
const commitResponse = await fetch('/api/commit-changes', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    content: modifiedContent,
    target_file: '/path/to/output.md'
  })
});

const commitResult = await commitResponse.json();
```

## Testing

### Unit Tests

```bash
# Run handler tests
cargo test handlers::diff_

# Run integration tests  
cargo test --test diff_viewer_integration
```

### Manual Testing

```bash
# Test health endpoint
curl http://localhost:8080/api/health

# Test diff viewer page
curl -I http://localhost:8080/diff

# Test diff data with non-existent files
curl "http://localhost:8080/api/diff-data?file1=/nonexistent.md"
```

## Performance Considerations

- **File Size Limits**: Large files may cause memory issues
- **Diff Algorithm**: O(n²) complexity for line comparison
- **Git Operations**: Can be slow for large repositories
- **Concurrent Access**: Multiple simultaneous diffs may impact performance

## Future Enhancements

- [ ] Authentication and authorization
- [ ] File upload support
- [ ] Advanced diff algorithms (word-level, semantic)
- [ ] Diff export formats (PDF, HTML)
- [ ] Real-time collaboration
- [ ] Conflict resolution tools
- [ ] Integration with external version control systems
