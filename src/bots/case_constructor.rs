use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::bots::{BotRegistry, BotSpecialty, BotInput, BotOutput, BotCollaboration, CollaborationStrategy, TaskContext};
use crate::db::DbPool;
use sqlx::Row;

/// Case construction workflow orchestrator
#[derive(Debug)]
pub struct CaseConstructor {
    pub case_id: Uuid,
    pub case_title: String,
    pub case_type: CaseType,
    pub team_members: RwLock<Vec<TeamMember>>,
    pub documents: RwLock<Vec<CaseDocument>>,
    pub workflow_stages: RwLock<Vec<WorkflowStage>>,
    pub bot_registry: Arc<BotRegistry>,
    pub db_pool: DbPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseType {
    FamilyLaw,
    ProfessionalResponsibility,
    AdaCompliance,
    Constitutional,
    Appeals,
    Emergency,
}

/// Team member in the case construction process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub role: TeamRole,
    pub specialties: Vec<BotSpecialty>,
    pub permissions: TeamPermissions,
    pub added_at: DateTime<Utc>,
    pub status: MemberStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamRole {
    LeadAttorney,
    AssociateAttorney,
    Paralegal,
    LegalAssistant,
    Expert,
    Consultant,
    Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPermissions {
    pub can_upload_documents: bool,
    pub can_review_documents: bool,
    pub can_approve_filings: bool,
    pub can_assign_tasks: bool,
    pub can_access_confidential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberStatus {
    Active,
    Pending,
    Suspended,
    Removed,
}

/// Document in case construction workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDocument {
    pub id: Uuid,
    pub filename: String,
    pub file_path: String,
    pub document_type: DocumentType,
    pub upload_date: DateTime<Utc>,
    pub uploaded_by: Uuid,
    pub file_size: u64,
    pub sha256_hash: String,
    pub review_status: DocumentReviewStatus,
    pub processing_results: Vec<ProcessingResult>,
    pub confidentiality_level: ConfidentialityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Motion,
    Brief,
    Evidence,
    Exhibit,
    Correspondence,
    CourtOrder,
    Stipulation,
    Contract,
    Medical,
    Financial,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentReviewStatus {
    Pending,
    InReview,
    NeedsRevision,
    Approved,
    Rejected,
    ReadyForFiling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfidentialityLevel {
    Public,
    Confidential,
    AttorneyClientPrivilege,
    WorkProduct,
    Sealed,
}

/// Processing result from bot analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub bot_id: Uuid,
    pub bot_specialty: BotSpecialty,
    pub analysis_type: String,
    pub result: serde_json::Value,
    pub confidence: f64,
    pub issues_found: Vec<String>,
    pub suggestions: Vec<String>,
    pub processed_at: DateTime<Utc>,
}

/// Workflow stage in case construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStage {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub stage_type: StageType,
    pub required_bots: Vec<BotSpecialty>,
    pub dependencies: Vec<Uuid>, // Other stage IDs that must complete first
    pub status: StageStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
    pub deliverables: Vec<Deliverable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageType {
    DocumentReview,
    LegalResearch,
    MotionDrafting,
    EvidenceAnalysis,
    TimelineConstruction,
    RiskAssessment,
    FilingPreparation,
    QualityControl,
    ClientReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageStatus {
    NotStarted,
    InProgress,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub name: String,
    pub description: String,
    pub document_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
}

impl CaseConstructor {
    pub fn new(
        case_title: String,
        case_type: CaseType,
        bot_registry: Arc<BotRegistry>,
        db_pool: DbPool,
    ) -> Self {
        Self {
            case_id: Uuid::new_v4(),
            case_title,
            case_type,
            team_members: RwLock::new(Vec::new()),
            documents: RwLock::new(Vec::new()),
            workflow_stages: RwLock::new(Vec::new()),
            bot_registry,
            db_pool,
        }
    }
    
    /// Add team member to the case
    pub async fn invite_team_member(&self, member: TeamMember) -> Result<(), CaseConstructorError> {
        // Insert into database
        sqlx::query!(
            "INSERT INTO case_team_members (id, case_id, name, role, permissions, status, added_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            member.id.to_string(),
            self.case_id.to_string(),
            member.name,
            serde_json::to_string(&member.role)?,
            serde_json::to_string(&member.permissions)?,
            serde_json::to_string(&member.status)?,
            member.added_at.to_rfc3339()
        )
        .execute(&self.db_pool)
        .await?;
        
        // Add to memory
        self.team_members.write().await.push(member);
        
        Ok(())
    }
    
    /// Upload document for case construction
    pub async fn upload_document(
        &self,
        filename: String,
        file_path: String,
        document_type: DocumentType,
        uploaded_by: Uuid,
        file_data: Vec<u8>,
    ) -> Result<Uuid, CaseConstructorError> {
        use sha2::{Sha256, Digest};
        
        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(&file_data);
        let hash = hex::encode(hasher.finalize());
        
        let document = CaseDocument {
            id: Uuid::new_v4(),
            filename,
            file_path: file_path.clone(),
            document_type: document_type.clone(),
            upload_date: Utc::now(),
            uploaded_by,
            file_size: file_data.len() as u64,
            sha256_hash: hash.clone(),
            review_status: DocumentReviewStatus::Pending,
            processing_results: Vec::new(),
            confidentiality_level: ConfidentialityLevel::Confidential,
        };
        
        // Save to database
        sqlx::query!(
            "INSERT INTO case_documents 
             (id, case_id, filename, file_path, document_type, upload_date, uploaded_by, 
              file_size, sha256_hash, review_status, confidentiality_level) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            document.id.to_string(),
            self.case_id.to_string(),
            document.filename,
            document.file_path,
            serde_json::to_string(&document.document_type)?,
            document.upload_date.to_rfc3339(),
            document.uploaded_by.to_string(),
            document.file_size as i64,
            document.sha256_hash,
            serde_json::to_string(&document.review_status)?,
            serde_json::to_string(&document.confidentiality_level)?
        )
        .execute(&self.db_pool)
        .await?;
        
        // Write file to disk
        tokio::fs::write(&file_path, &file_data).await?;
        
        let document_id = document.id;
        self.documents.write().await.push(document);
        
        // Queue for automated processing
        self.queue_document_processing(document_id).await?;
        
        Ok(document_id)
    }
    
    /// Queue document for bot processing
    async fn queue_document_processing(&self, document_id: Uuid) -> Result<(), CaseConstructorError> {
        let documents = self.documents.read().await;
        let document = documents.iter()
            .find(|d| d.id == document_id)
            .ok_or_else(|| CaseConstructorError::DocumentNotFound(document_id))?;
        
        // Determine which bots should process this document
        let bots_to_use = match &document.document_type {
            DocumentType::Motion => vec![
                BotSpecialty::DocumentReviewAssistant,
                BotSpecialty::CitationValidator,
                BotSpecialty::LegalDocumentGenerator,
            ],
            DocumentType::Evidence => vec![
                BotSpecialty::EvidenceOrganizer,
                BotSpecialty::TimelineAnalyst,
                BotSpecialty::DocumentReviewAssistant,
            ],
            DocumentType::Correspondence => vec![
                BotSpecialty::ClientCommunicationLogger,
                BotSpecialty::DocumentReviewAssistant,
            ],
            _ => vec![BotSpecialty::DocumentReviewAssistant],
        };
        
        // Create processing tasks
        for bot_specialty in bots_to_use {
            let task = BotInput {
                task_id: Uuid::new_v4(),
                task_type: "document_analysis".to_string(),
                data: serde_json::json!({
                    "document_id": document.id,
                    "filename": document.filename,
                    "document_type": document.document_type,
                    "file_path": document.file_path
                }),
                context: HashMap::from([
                    ("case_id".to_string(), self.case_id.to_string()),
                    ("case_type".to_string(), serde_json::to_string(&self.case_type)?),
                ]),
                priority: 128, // Medium priority
                deadline: None,
                requester: "case_constructor".to_string(),
            };
            
            self.bot_registry.queue_task(task).await;
        }
        
        Ok(())
    }
    
    /// Process bot results and update document
    pub async fn process_bot_results(&self, document_id: Uuid, result: BotOutput) -> Result<(), CaseConstructorError> {
        let processing_result = ProcessingResult {
            bot_id: result.bot_id,
            bot_specialty: BotSpecialty::DocumentReviewAssistant, // This should come from the bot
            analysis_type: "document_analysis".to_string(),
            result: result.result,
            confidence: result.confidence,
            issues_found: extract_issues(&result),
            suggestions: result.recommendations,
            processed_at: Utc::now(),
        };
        
        // Update document in memory
        let mut documents = self.documents.write().await;
        if let Some(document) = documents.iter_mut().find(|d| d.id == document_id) {
            document.processing_results.push(processing_result.clone());
            
            // Update review status based on results
            if processing_result.issues_found.is_empty() && processing_result.confidence > 0.8 {
                document.review_status = DocumentReviewStatus::Approved;
            } else if !processing_result.issues_found.is_empty() {
                document.review_status = DocumentReviewStatus::NeedsRevision;
            }
        }
        
        // Save to database
        sqlx::query!(
            "INSERT INTO document_processing_results 
             (id, document_id, bot_id, analysis_type, result, confidence, processed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            Uuid::new_v4().to_string(),
            document_id.to_string(),
            processing_result.bot_id.to_string(),
            processing_result.analysis_type,
            serde_json::to_string(&processing_result.result)?,
            processing_result.confidence,
            processing_result.processed_at.to_rfc3339()
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Create workflow stages for case type
    pub async fn initialize_workflow(&self) -> Result<(), CaseConstructorError> {
        let stages = match self.case_type {
            CaseType::FamilyLaw => self.create_family_law_workflow().await?,
            CaseType::ProfessionalResponsibility => self.create_ethics_workflow().await?,
            CaseType::Emergency => self.create_emergency_workflow().await?,
            _ => self.create_standard_workflow().await?,
        };
        
        for stage in stages {
            // Save to database
            sqlx::query!(
                "INSERT INTO workflow_stages 
                 (id, case_id, name, description, stage_type, status, required_bots)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                stage.id.to_string(),
                self.case_id.to_string(),
                stage.name,
                stage.description,
                serde_json::to_string(&stage.stage_type)?,
                serde_json::to_string(&stage.status)?,
                serde_json::to_string(&stage.required_bots)?
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        *self.workflow_stages.write().await = stages;
        Ok(())
    }
    
    /// Create family law specific workflow
    async fn create_family_law_workflow(&self) -> Result<Vec<WorkflowStage>, CaseConstructorError> {
        Ok(vec![
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Case Intake & Assessment".to_string(),
                description: "Initial case evaluation and client interview".to_string(),
                stage_type: StageType::DocumentReview,
                required_bots: vec![
                    BotSpecialty::ParentalFitnessEvaluator,
                    BotSpecialty::AdaComplianceAnalyzer,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![
                    Deliverable {
                        name: "Client Intake Form".to_string(),
                        description: "Completed client information and case details".to_string(),
                        document_id: None,
                        due_date: Some(Utc::now() + chrono::Duration::days(3)),
                        completed: false,
                    }
                ],
            },
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Evidence Collection".to_string(),
                description: "Gather all relevant documents and evidence".to_string(),
                stage_type: StageType::EvidenceAnalysis,
                required_bots: vec![
                    BotSpecialty::EvidenceOrganizer,
                    BotSpecialty::PlacementViolationTracker,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Timeline Construction".to_string(),
                description: "Build comprehensive case timeline".to_string(),
                stage_type: StageType::TimelineConstruction,
                required_bots: vec![
                    BotSpecialty::TimelineAnalyst,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Motion Drafting".to_string(),
                description: "Draft legal motions and pleadings".to_string(),
                stage_type: StageType::MotionDrafting,
                required_bots: vec![
                    BotSpecialty::MotionDrafter,
                    BotSpecialty::CitationValidator,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
        ])
    }
    
    /// Create ethics workflow for professional responsibility cases
    async fn create_ethics_workflow(&self) -> Result<Vec<WorkflowStage>, CaseConstructorError> {
        Ok(vec![
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Ethics Investigation".to_string(),
                description: "Investigate potential ethics violations".to_string(),
                stage_type: StageType::LegalResearch,
                required_bots: vec![
                    BotSpecialty::AttorneyMisconductInvestigator,
                    BotSpecialty::EthicsRuleAnalyzer,
                    BotSpecialty::UnauthorizedPracticeMonitor,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
        ])
    }
    
    /// Create emergency workflow
    async fn create_emergency_workflow(&self) -> Result<Vec<WorkflowStage>, CaseConstructorError> {
        Ok(vec![
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Emergency Assessment".to_string(),
                description: "Rapid assessment of emergency situation".to_string(),
                stage_type: StageType::RiskAssessment,
                required_bots: vec![
                    BotSpecialty::EmergencyMotion,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
        ])
    }
    
    /// Create standard workflow
    async fn create_standard_workflow(&self) -> Result<Vec<WorkflowStage>, CaseConstructorError> {
        Ok(vec![
            WorkflowStage {
                id: Uuid::new_v4(),
                name: "Document Review".to_string(),
                description: "Review all case documents".to_string(),
                stage_type: StageType::DocumentReview,
                required_bots: vec![
                    BotSpecialty::DocumentReviewAssistant,
                ],
                dependencies: vec![],
                status: StageStatus::NotStarted,
                started_at: None,
                completed_at: None,
                assigned_to: None,
                deliverables: vec![],
            },
        ])
    }
    
    /// Get case summary for dashboard
    pub async fn get_case_summary(&self) -> CaseSummary {
        let team_members = self.team_members.read().await;
        let documents = self.documents.read().await;
        let workflow_stages = self.workflow_stages.read().await;
        
        CaseSummary {
            case_id: self.case_id,
            title: self.case_title.clone(),
            case_type: self.case_type.clone(),
            team_size: team_members.len(),
            document_count: documents.len(),
            stages_completed: workflow_stages.iter()
                .filter(|s| s.status == StageStatus::Completed)
                .count(),
            total_stages: workflow_stages.len(),
            next_deadline: workflow_stages.iter()
                .filter_map(|s| s.deliverables.iter()
                    .filter_map(|d| d.due_date)
                    .min())
                .min(),
        }
    }
}

/// Case summary for dashboard display
#[derive(Debug, Serialize, Deserialize)]
pub struct CaseSummary {
    pub case_id: Uuid,
    pub title: String,
    pub case_type: CaseType,
    pub team_size: usize,
    pub document_count: usize,
    pub stages_completed: usize,
    pub total_stages: usize,
    pub next_deadline: Option<DateTime<Utc>>,
}

/// Extract issues from bot output
fn extract_issues(output: &BotOutput) -> Vec<String> {
    // This would parse the bot result to extract specific issues
    if output.confidence < 0.5 {
        vec!["Low confidence in analysis".to_string()]
    } else {
        vec![]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CaseConstructorError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] tokio::io::Error),
    #[error("Document not found: {0}")]
    DocumentNotFound(Uuid),
    #[error("Team member not found: {0}")]
    TeamMemberNotFound(Uuid),
    #[error("Invalid permissions")]
    InvalidPermissions,
}
