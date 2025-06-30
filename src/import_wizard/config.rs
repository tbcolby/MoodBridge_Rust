use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::import_wizard::classifier::{FileCategory, SensitivityLevel};
use crate::import_wizard::crypto::EncryptionStandard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWizardConfig {
    pub storage_paths: StoragePaths,
    pub encryption_settings: EncryptionSettings,
    pub classification_rules: ClassificationRules,
    pub processing_options: ProcessingOptions,
    pub security_policies: SecurityPolicies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePaths {
    pub base_path: PathBuf,
    pub encrypted_path: PathBuf,
    pub temp_path: PathBuf,
    pub backup_path: PathBuf,
    pub quarantine_path: PathBuf,
    pub category_paths: HashMap<FileCategory, PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    pub default_standard: EncryptionStandard,
    pub sensitivity_mapping: HashMap<SensitivityLevel, EncryptionStandard>,
    pub key_derivation_iterations: HashMap<EncryptionStandard, u32>,
    pub enable_compression: bool,
    pub enable_integrity_check: bool,
    pub secure_delete_originals: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationRules {
    pub auto_classify: bool,
    pub custom_keywords: HashMap<String, FileCategory>,
    pub sensitivity_keywords: HashMap<String, SensitivityLevel>,
    pub file_size_limits: HashMap<FileCategory, u64>, // Max size in bytes
    pub allowed_extensions: HashMap<FileCategory, Vec<String>>,
    pub blocked_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingOptions {
    pub max_concurrent_imports: usize,
    pub max_file_size: u64, // Maximum file size in bytes
    pub scan_for_malware: bool,
    pub extract_metadata: bool,
    pub generate_thumbnails: bool,
    pub auto_organize: bool,
    pub duplicate_handling: DuplicateHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DuplicateHandling {
    Skip,
    Overwrite,
    Rename,
    MoveToQuarantine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicies {
    pub require_approval: HashMap<SensitivityLevel, bool>,
    pub access_controls: HashMap<FileCategory, Vec<String>>, // Roles that can access
    pub audit_logging: bool,
    pub data_retention_days: HashMap<FileCategory, u32>,
    pub auto_deletion: bool,
    pub compliance_standards: Vec<ComplianceStandard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStandard {
    GDPR,
    HIPAA,
    SOX,
    CCPA,
    ISO27001,
    NIST,
    PCI_DSS,
}

impl Default for ImportWizardConfig {
    fn default() -> Self {
        let base_path = PathBuf::from("data/moodbridge");
        
        let mut category_paths = HashMap::new();
        category_paths.insert(FileCategory::LegalDocument, base_path.join("legal"));
        category_paths.insert(FileCategory::FinancialDocument, base_path.join("financial"));
        category_paths.insert(FileCategory::PersonalDocument, base_path.join("personal"));
        category_paths.insert(FileCategory::MediaFile, base_path.join("media"));
        category_paths.insert(FileCategory::CodeFile, base_path.join("code"));
        category_paths.insert(FileCategory::Archive, base_path.join("archives"));
        category_paths.insert(FileCategory::Configuration, base_path.join("config"));
        category_paths.insert(FileCategory::Database, base_path.join("databases"));
        category_paths.insert(FileCategory::Unknown, base_path.join("unknown"));

        let mut sensitivity_mapping = HashMap::new();
        sensitivity_mapping.insert(SensitivityLevel::Public, EncryptionStandard::None);
        sensitivity_mapping.insert(SensitivityLevel::Internal, EncryptionStandard::None);
        sensitivity_mapping.insert(SensitivityLevel::Confidential, EncryptionStandard::AES128GCM);
        sensitivity_mapping.insert(SensitivityLevel::HighlyConfidential, EncryptionStandard::AES256GCM);
        sensitivity_mapping.insert(SensitivityLevel::TopSecret, EncryptionStandard::AES256GCM);

        let mut key_derivation_iterations = HashMap::new();
        key_derivation_iterations.insert(EncryptionStandard::AES128GCM, 25_000);
        key_derivation_iterations.insert(EncryptionStandard::AES256GCM, 100_000);
        key_derivation_iterations.insert(EncryptionStandard::ChaCha20Poly1305, 50_000);

        let mut custom_keywords = HashMap::new();
        custom_keywords.insert("contract".to_string(), FileCategory::LegalDocument);
        custom_keywords.insert("invoice".to_string(), FileCategory::FinancialDocument);
        custom_keywords.insert("agreement".to_string(), FileCategory::LegalDocument);
        custom_keywords.insert("receipt".to_string(), FileCategory::FinancialDocument);

        let mut sensitivity_keywords = HashMap::new();
        sensitivity_keywords.insert("confidential".to_string(), SensitivityLevel::HighlyConfidential);
        sensitivity_keywords.insert("secret".to_string(), SensitivityLevel::TopSecret);
        sensitivity_keywords.insert("private".to_string(), SensitivityLevel::Confidential);
        sensitivity_keywords.insert("internal".to_string(), SensitivityLevel::Internal);

        let mut file_size_limits = HashMap::new();
        file_size_limits.insert(FileCategory::LegalDocument, 100 * 1024 * 1024); // 100MB
        file_size_limits.insert(FileCategory::FinancialDocument, 50 * 1024 * 1024); // 50MB
        file_size_limits.insert(FileCategory::MediaFile, 1024 * 1024 * 1024); // 1GB
        file_size_limits.insert(FileCategory::Archive, 500 * 1024 * 1024); // 500MB

        let mut allowed_extensions = HashMap::new();
        allowed_extensions.insert(FileCategory::LegalDocument, vec![
            "pdf".to_string(), "doc".to_string(), "docx".to_string(), "txt".to_string()
        ]);
        allowed_extensions.insert(FileCategory::FinancialDocument, vec![
            "pdf".to_string(), "xls".to_string(), "xlsx".to_string(), "csv".to_string()
        ]);
        allowed_extensions.insert(FileCategory::MediaFile, vec![
            "jpg".to_string(), "png".to_string(), "mp4".to_string(), "mp3".to_string()
        ]);

        let mut require_approval = HashMap::new();
        require_approval.insert(SensitivityLevel::TopSecret, true);
        require_approval.insert(SensitivityLevel::HighlyConfidential, true);
        require_approval.insert(SensitivityLevel::Confidential, false);

        let mut access_controls = HashMap::new();
        access_controls.insert(FileCategory::LegalDocument, vec!["legal_team".to_string(), "admin".to_string()]);
        access_controls.insert(FileCategory::FinancialDocument, vec!["finance_team".to_string(), "admin".to_string()]);

        let mut data_retention_days = HashMap::new();
        data_retention_days.insert(FileCategory::LegalDocument, 2555); // 7 years
        data_retention_days.insert(FileCategory::FinancialDocument, 1825); // 5 years

        ImportWizardConfig {
            storage_paths: StoragePaths {
                base_path: base_path.clone(),
                encrypted_path: base_path.join("encrypted"),
                temp_path: base_path.join("temp"),
                backup_path: base_path.join("backup"),
                quarantine_path: base_path.join("quarantine"),
                category_paths,
            },
            encryption_settings: EncryptionSettings {
                default_standard: EncryptionStandard::AES256GCM,
                sensitivity_mapping,
                key_derivation_iterations,
                enable_compression: true,
                enable_integrity_check: true,
                secure_delete_originals: true,
            },
            classification_rules: ClassificationRules {
                auto_classify: true,
                custom_keywords,
                sensitivity_keywords,
                file_size_limits,
                allowed_extensions,
                blocked_extensions: vec![
                    "exe".to_string(), "bat".to_string(), "com".to_string(), 
                    "scr".to_string(), "vbs".to_string(), "js".to_string()
                ],
            },
            processing_options: ProcessingOptions {
                max_concurrent_imports: 5,
                max_file_size: 1024 * 1024 * 1024, // 1GB
                scan_for_malware: true,
                extract_metadata: true,
                generate_thumbnails: true,
                auto_organize: true,
                duplicate_handling: DuplicateHandling::Rename,
            },
            security_policies: SecurityPolicies {
                require_approval,
                access_controls,
                audit_logging: true,
                data_retention_days,
                auto_deletion: false,
                compliance_standards: vec![
                    ComplianceStandard::GDPR,
                    ComplianceStandard::HIPAA,
                    ComplianceStandard::SOX,
                ],
            },
        }
    }
}

impl ImportWizardConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_file(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = std::fs::read_to_string(config_path)?;
        let config: ImportWizardConfig = serde_json::from_str(&config_content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_content)?;
        Ok(())
    }

    pub fn get_storage_path(&self, category: &FileCategory) -> PathBuf {
        self.storage_paths.category_paths
            .get(category)
            .cloned()
            .unwrap_or_else(|| self.storage_paths.base_path.join("unknown"))
    }

    pub fn get_encryption_standard(&self, sensitivity: &SensitivityLevel) -> EncryptionStandard {
        self.encryption_settings.sensitivity_mapping
            .get(sensitivity)
            .cloned()
            .unwrap_or(self.encryption_settings.default_standard.clone())
    }

    pub fn is_extension_allowed(&self, category: &FileCategory, extension: &str) -> bool {
        // Check if extension is blocked globally
        if self.classification_rules.blocked_extensions.contains(&extension.to_lowercase()) {
            return false;
        }

        // Check if extension is allowed for this category
        if let Some(allowed) = self.classification_rules.allowed_extensions.get(category) {
            allowed.contains(&extension.to_lowercase())
        } else {
            true // If no specific rules, allow by default
        }
    }

    pub fn requires_approval(&self, sensitivity: &SensitivityLevel) -> bool {
        self.security_policies.require_approval
            .get(sensitivity)
            .copied()
            .unwrap_or(false)
    }

    pub fn get_max_file_size(&self, category: &FileCategory) -> u64 {
        self.classification_rules.file_size_limits
            .get(category)
            .copied()
            .unwrap_or(self.processing_options.max_file_size)
    }

    pub fn ensure_directories(&self) -> Result<(), std::io::Error> {
        // Create all necessary directories
        std::fs::create_dir_all(&self.storage_paths.base_path)?;
        std::fs::create_dir_all(&self.storage_paths.encrypted_path)?;
        std::fs::create_dir_all(&self.storage_paths.temp_path)?;
        std::fs::create_dir_all(&self.storage_paths.backup_path)?;
        std::fs::create_dir_all(&self.storage_paths.quarantine_path)?;

        // Create category directories
        for path in self.storage_paths.category_paths.values() {
            std::fs::create_dir_all(path)?;
        }

        Ok(())
    }
}
