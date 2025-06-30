use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::import_wizard::metadata::FileMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileCategory {
    LegalDocument,
    FinancialDocument,
    PersonalDocument,
    MediaFile,
    CodeFile,
    Archive,
    Configuration,
    Database,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitivityLevel {
    Public,
    Internal,
    Confidential,
    HighlyConfidential,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileClassification {
    pub category: FileCategory,
    pub sensitivity: SensitivityLevel,
    pub requires_encryption: bool,
    pub retention_period_days: Option<u32>,
    pub access_level: u8, // 1-10 scale
}

pub fn classify_file(file_path: &str, metadata: &FileMetadata) -> FileClassification {
    let path = Path::new(file_path);
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let filename = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    
    // Advanced classification based on multiple factors
    let category = classify_by_extension_and_content(extension, filename, metadata);
    let sensitivity = determine_sensitivity(filename, &category, metadata);
    let requires_encryption = should_encrypt(&category, &sensitivity);
    let retention_period = get_retention_period(&category, &sensitivity);
    let access_level = calculate_access_level(&category, &sensitivity);
    
    FileClassification {
        category,
        sensitivity,
        requires_encryption,
        retention_period_days: retention_period,
        access_level,
    }
}

fn classify_by_extension_and_content(extension: &str, filename: &str, metadata: &FileMetadata) -> FileCategory {
    // Legal document patterns
    if is_legal_document(extension, filename) {
        return FileCategory::LegalDocument;
    }
    
    // Financial document patterns
    if is_financial_document(extension, filename) {
        return FileCategory::FinancialDocument;
    }
    
    match extension.to_lowercase().as_str() {
        // Code files
        "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "h" | "go" | "rb" | "php" => FileCategory::CodeFile,
        
        // Media files
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" |
        "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" |
        "mp3" | "wav" | "flac" | "aac" | "ogg" => FileCategory::MediaFile,
        
        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => FileCategory::Archive,
        
        // Configuration
        "json" | "yaml" | "yml" | "toml" | "ini" | "conf" | "config" => FileCategory::Configuration,
        
        // Database
        "db" | "sqlite" | "sqlite3" | "mdb" | "accdb" => FileCategory::Database,
        
        // Default to personal document for common office formats
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" => {
            if is_legal_document(extension, filename) {
                FileCategory::LegalDocument
            } else if is_financial_document(extension, filename) {
                FileCategory::FinancialDocument
            } else {
                FileCategory::PersonalDocument
            }
        }
        
        _ => FileCategory::Unknown,
    }
}

fn is_legal_document(extension: &str, filename: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    
    // Legal document keywords
    let legal_keywords = [
        "contract", "agreement", "legal", "law", "court", "brief", "motion",
        "deposition", "affidavit", "complaint", "answer", "discovery",
        "subpoena", "settlement", "judgment", "order", "decree", "will",
        "trust", "patent", "trademark", "copyright", "license", "nda",
        "confidentiality", "terms", "conditions", "policy", "compliance"
    ];
    
    legal_keywords.iter().any(|keyword| filename_lower.contains(keyword))
}

fn is_financial_document(extension: &str, filename: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    
    // Financial document keywords
    let financial_keywords = [
        "invoice", "receipt", "statement", "balance", "financial", "tax",
        "budget", "expense", "income", "payroll", "salary", "bank",
        "account", "transaction", "payment", "billing", "cost", "profit",
        "revenue", "audit", "accounting", "bookkeeping", "ledger"
    ];
    
    financial_keywords.iter().any(|keyword| filename_lower.contains(keyword))
}

fn determine_sensitivity(filename: &str, category: &FileCategory, metadata: &FileMetadata) -> SensitivityLevel {
    let filename_lower = filename.to_lowercase();
    
    // High sensitivity keywords
    let top_secret_keywords = ["classified", "secret", "confidential", "restricted", "private"];
    let confidential_keywords = ["internal", "proprietary", "sensitive", "personal", "ssn", "social"];
    
    if top_secret_keywords.iter().any(|keyword| filename_lower.contains(keyword)) {
        return SensitivityLevel::TopSecret;
    }
    
    if confidential_keywords.iter().any(|keyword| filename_lower.contains(keyword)) {
        return SensitivityLevel::HighlyConfidential;
    }
    
    // Category-based sensitivity
    match category {
        FileCategory::LegalDocument => SensitivityLevel::HighlyConfidential,
        FileCategory::FinancialDocument => SensitivityLevel::Confidential,
        FileCategory::PersonalDocument => SensitivityLevel::Internal,
        FileCategory::CodeFile => SensitivityLevel::Internal,
        FileCategory::Database => SensitivityLevel::HighlyConfidential,
        FileCategory::Configuration => SensitivityLevel::Confidential,
        _ => SensitivityLevel::Public,
    }
}

fn should_encrypt(category: &FileCategory, sensitivity: &SensitivityLevel) -> bool {
    match sensitivity {
        SensitivityLevel::TopSecret | SensitivityLevel::HighlyConfidential => true,
        SensitivityLevel::Confidential => {
            matches!(category, FileCategory::LegalDocument | FileCategory::FinancialDocument | FileCategory::Database)
        }
        _ => false,
    }
}

fn get_retention_period(category: &FileCategory, sensitivity: &SensitivityLevel) -> Option<u32> {
    match (category, sensitivity) {
        (FileCategory::LegalDocument, _) => Some(2555), // 7 years
        (FileCategory::FinancialDocument, _) => Some(1825), // 5 years
        (_, SensitivityLevel::TopSecret) => Some(3650), // 10 years
        (_, SensitivityLevel::HighlyConfidential) => Some(1825), // 5 years
        _ => None,
    }
}

fn calculate_access_level(category: &FileCategory, sensitivity: &SensitivityLevel) -> u8 {
    let base_level = match category {
        FileCategory::LegalDocument => 8,
        FileCategory::FinancialDocument => 7,
        FileCategory::Database => 9,
        FileCategory::Configuration => 6,
        FileCategory::PersonalDocument => 5,
        FileCategory::CodeFile => 4,
        FileCategory::Archive => 3,
        FileCategory::MediaFile => 2,
        FileCategory::Unknown => 1,
    };
    
    let sensitivity_modifier = match sensitivity {
        SensitivityLevel::TopSecret => 2,
        SensitivityLevel::HighlyConfidential => 1,
        SensitivityLevel::Confidential => 0,
        SensitivityLevel::Internal => -1,
        SensitivityLevel::Public => -2,
    };
    
    (base_level + sensitivity_modifier).clamp(1, 10) as u8
}

