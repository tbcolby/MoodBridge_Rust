use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub checksum: String,
    pub permissions: u32,
    pub is_hidden: bool,
    pub content_preview: Option<String>, // First 500 chars for text files
}

impl FileMetadata {
    pub async fn from_file(file_path: &str) -> Result<Self, AppError> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(AppError::ValidationError(format!("File does not exist: {}", file_path)));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| AppError::InternalError(format!("Failed to read metadata: {}", e)))?;

        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        let created_at = metadata.created()
            .map(|time| DateTime::<Utc>::from(time))
            .unwrap_or_else(|_| Utc::now());

        let modified_at = metadata.modified()
            .map(|time| DateTime::<Utc>::from(time))
            .unwrap_or_else(|_| Utc::now());

        // Calculate checksum for file integrity
        let checksum = calculate_file_checksum(file_path)?;

        // Get MIME type based on extension
        let mime_type = get_mime_type(&extension);

        // Check if file is hidden (starts with .)
        let is_hidden = file_name.starts_with('.');

        // Get content preview for text files
        let content_preview = get_content_preview(file_path, &mime_type).await;

        // Get file permissions (Unix-style)
        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        
        #[cfg(not(unix))]
        let permissions = 0o644; // Default permissions for non-Unix systems

        Ok(FileMetadata {
            file_path: file_path.to_string(),
            file_name,
            file_size: metadata.len(),
            created_at,
            modified_at,
            extension,
            mime_type,
            checksum,
            permissions,
            is_hidden,
            content_preview,
        })
    }
}

fn calculate_file_checksum(file_path: &str) -> Result<String, AppError> {
    use ring::digest::{Context, SHA256};
    use std::io::Read;

    let mut file = fs::File::open(file_path)
        .map_err(|e| AppError::InternalError(format!("Failed to open file for checksum: {}", e)))?;

    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer)
            .map_err(|e| AppError::InternalError(format!("Failed to read file for checksum: {}", e)))?;
        
        if count == 0 {
            break;
        }
        
        context.update(&buffer[..count]);
    }

    let digest = context.finish();
    Ok(hex::encode(digest.as_ref()))
}

fn get_mime_type(extension: &Option<String>) -> Option<String> {
    match extension.as_deref() {
        Some("txt") => Some("text/plain".to_string()),
        Some("pdf") => Some("application/pdf".to_string()),
        Some("doc") => Some("application/msword".to_string()),
        Some("docx") => Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string()),
        Some("xls") => Some("application/vnd.ms-excel".to_string()),
        Some("xlsx") => Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()),
        Some("jpg") | Some("jpeg") => Some("image/jpeg".to_string()),
        Some("png") => Some("image/png".to_string()),
        Some("gif") => Some("image/gif".to_string()),
        Some("mp4") => Some("video/mp4".to_string()),
        Some("mp3") => Some("audio/mpeg".to_string()),
        Some("json") => Some("application/json".to_string()),
        Some("xml") => Some("application/xml".to_string()),
        Some("html") => Some("text/html".to_string()),
        Some("css") => Some("text/css".to_string()),
        Some("js") => Some("application/javascript".to_string()),
        Some("zip") => Some("application/zip".to_string()),
        _ => None,
    }
}

async fn get_content_preview(file_path: &str, mime_type: &Option<String>) -> Option<String> {
    // Only preview text-based files
    let is_text = mime_type.as_ref()
        .map(|mime| mime.starts_with("text/") || mime.contains("json") || mime.contains("xml"))
        .unwrap_or(false);

    if !is_text {
        return None;
    }

    // Read first 500 characters for preview
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let preview = content.chars().take(500).collect::<String>();
            Some(preview)
        }
        Err(_) => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub metadata: FileMetadata,
    pub contains_pii: bool,
    pub contains_financial_data: bool,
    pub contains_legal_terms: bool,
    pub language: Option<String>,
    pub word_count: Option<u32>,
    pub line_count: Option<u32>,
}

impl FileAnalysis {
    pub async fn analyze_file(file_path: &str) -> Result<Self, AppError> {
        let metadata = FileMetadata::from_file(file_path).await?;
        
        let (contains_pii, contains_financial_data, contains_legal_terms, language, word_count, line_count) = 
            if let Some(preview) = &metadata.content_preview {
                (
                    detect_pii(preview),
                    detect_financial_data(preview),
                    detect_legal_terms(preview),
                    detect_language(preview),
                    Some(count_words(preview)),
                    Some(count_lines(preview)),
                )
            } else {
                (false, false, false, None, None, None)
            };

        Ok(FileAnalysis {
            metadata,
            contains_pii,
            contains_financial_data,
            contains_legal_terms,
            language,
            word_count,
            line_count,
        })
    }
}

fn detect_pii(content: &str) -> bool {
    let content_lower = content.to_lowercase();
    
    // Common PII patterns
    let pii_patterns = [
        "social security", "ssn", "date of birth", "dob", "driver's license",
        "passport", "credit card", "bank account", "routing number",
        "personal identification", "national id", "tax id", "ein"
    ];
    
    pii_patterns.iter().any(|pattern| content_lower.contains(pattern))
}

fn detect_financial_data(content: &str) -> bool {
    let content_lower = content.to_lowercase();
    
    let financial_patterns = [
        "account number", "balance", "transaction", "payment",
        "invoice", "receipt", "tax", "irs", "w-2", "1099",
        "salary", "wage", "income", "expense", "budget"
    ];
    
    financial_patterns.iter().any(|pattern| content_lower.contains(pattern))
}

fn detect_legal_terms(content: &str) -> bool {
    let content_lower = content.to_lowercase();
    
    let legal_patterns = [
        "whereas", "therefore", "party", "agreement", "contract",
        "terms and conditions", "liability", "indemnity", "breach",
        "damages", "jurisdiction", "governing law", "dispute resolution"
    ];
    
    legal_patterns.iter().any(|pattern| content_lower.contains(pattern))
}

fn detect_language(content: &str) -> Option<String> {
    // Simple language detection based on common words
    // In a real implementation, use a proper language detection library
    
    let english_words = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by"];
    let spanish_words = ["el", "la", "y", "o", "pero", "en", "a", "para", "de", "con", "por"];
    let french_words = ["le", "la", "et", "ou", "mais", "dans", "à", "pour", "de", "avec", "par"];
    
    let content_lower = content.to_lowercase();
    
    let english_count = english_words.iter()
        .map(|word| content_lower.matches(word).count())
        .sum::<usize>();
    
    let spanish_count = spanish_words.iter()
        .map(|word| content_lower.matches(word).count())
        .sum::<usize>();
    
    let french_count = french_words.iter()
        .map(|word| content_lower.matches(word).count())
        .sum::<usize>();
    
    if english_count > spanish_count && english_count > french_count {
        Some("en".to_string())
    } else if spanish_count > french_count {
        Some("es".to_string())
    } else if french_count > 0 {
        Some("fr".to_string())
    } else {
        None
    }
}

fn count_words(content: &str) -> u32 {
    content.split_whitespace().count() as u32
}

fn count_lines(content: &str) -> u32 {
    content.lines().count() as u32
}
