use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use crate::ai::{AiService, AnalysisResponse, AiError};

pub mod legal_team;
pub mod case_constructor;
pub mod document_processor;
pub mod review_coordinator;
pub mod salesforce_cta_bot;
pub mod document_management_bot;
pub mod deadline_management_bot;
pub mod email_notification_bot;
pub mod analytics_reporting_bot;

/// Bot specialization types for legal work
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BotSpecialty {
    // Court Proceedings
    MotionDrafter,
    EvidenceOrganizer,
    TimelineAnalyst,
    PlacementViolationTracker,
    CourtFilingAssistant,
    EmergencyMotion,
    ContemptCalculator,
    
    // Family Law
    ChildPlacementOptimizer,
    ParentalFitnessEvaluator,
    CustodyMediatorMonitor,
    GuardianAdLitemCoordinator,
    
    // Professional Responsibility
    AttorneyMisconductInvestigator,
    EthicsRuleAnalyzer,
    UnauthorizedPracticeMonitor,
    
    // ADA & Disability
    AdaComplianceAnalyzer,
    MentalHealthLegalProtector,
    DisabilityAccommodationPlanner,
    
    // Document Management
    LegalDocumentGenerator,
    DocumentReviewAssistant,
    CitationValidator,
    DigitalSignatureValidator,
    
    // Research & Analysis
    CaseLawResearcher,
    StatuteInterpreter,
    PrecedentPatternAnalyzer,
    LegalTrendForecaster,
    
    // Case Management
    CaseStatusDashboard,
    ClientCommunicationLogger,
    LegalOperationsOrchestrator,
    
    // Enterprise Technology
    SalesforceArchitect,
    
    // Document and Workflow Management
    DocumentManagement,
    EmailNotificationBot,
    DeadlineManagement,
    LegalResearch,
    FormsAutomation,
    ClientCommunication,
    ContractAnalysis,
    WorkflowOptimization,
    DataMigration,
    AnalyticsReporting,
    ComplianceMonitoring,
    BillingAutomation,
    SecurityMonitoring,
    IntegrationManagement,
    UserActivityTracker,
    ApiManagement,
    AiPoweredSearch,
    CollaborationBot,
    ProjectManagement,
    KnowledgeBase,
}

/// Bot capability trait
#[async_trait::async_trait]
pub trait LegalBot {
    fn get_id(&self) -> Uuid;
    fn get_specialty(&self) -> BotSpecialty;
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_capabilities(&self) -> &[String];
    
    async fn analyze(&self, input: &BotInput) -> Result<BotOutput, BotError>;
    async fn can_handle(&self, task_type: &str) -> bool;
    fn get_priority(&self, task_type: &str) -> u8; // 0-255, higher = more priority
}

/// Input structure for bot processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotInput {
    pub task_id: Uuid,
    pub task_type: String,
    pub data: serde_json::Value,
    pub context: HashMap<String, String>,
    pub priority: u8,
    pub deadline: Option<DateTime<Utc>>,
    pub requester: String,
}

/// Output structure from bot processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotOutput {
    pub task_id: Uuid,
    pub bot_id: Uuid,
    pub success: bool,
    pub result: serde_json::Value,
    pub confidence: f64,
    pub recommendations: Vec<String>,
    pub next_actions: Vec<NextAction>,
    pub processing_time_ms: u128,
    pub error_message: Option<String>,
}

/// Next action recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction {
    pub action_type: String,
    pub description: String,
    pub priority: u8,
    pub suggested_bot: Option<BotSpecialty>,
    pub estimated_time_hours: Option<f32>,
}

/// Bot error types
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("AI service error: {0}")]
    AiServiceError(#[from] AiError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Timeout error")]
    TimeoutError,
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
}

/// Bot registry for managing the legal team
#[derive(Debug)]
pub struct BotRegistry {
    bots: RwLock<HashMap<Uuid, Arc<dyn LegalBot + Send + Sync>>>,
    specialty_index: RwLock<HashMap<BotSpecialty, Vec<Uuid>>>,
    task_queue: RwLock<Vec<BotInput>>,
    ai_service: Arc<dyn AiService + Send + Sync>,
}

impl BotRegistry {
    pub fn new(ai_service: Arc<dyn AiService + Send + Sync>) -> Self {
        Self {
            bots: RwLock::new(HashMap::new()),
            specialty_index: RwLock::new(HashMap::new()),
            task_queue: RwLock::new(Vec::new()),
            ai_service,
        }
    }
    
    /// Register a new bot with the system
    pub async fn register_bot(&self, bot: Arc<dyn LegalBot + Send + Sync>) {
        let bot_id = bot.get_id();
        let specialty = bot.get_specialty();
        
        // Add to main registry
        self.bots.write().await.insert(bot_id, bot);
        
        // Add to specialty index
        self.specialty_index.write().await
            .entry(specialty)
            .or_insert_with(Vec::new)
            .push(bot_id);
    }
    
    /// Get bots by specialty
    pub async fn get_bots_by_specialty(&self, specialty: &BotSpecialty) -> Vec<Arc<dyn LegalBot + Send + Sync>> {
        let specialty_index = self.specialty_index.read().await;
        let bot_registry = self.bots.read().await;
        
        if let Some(bot_ids) = specialty_index.get(specialty) {
            bot_ids.iter()
                .filter_map(|id| bot_registry.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Route task to best available bot
    pub async fn route_task(&self, task: BotInput) -> Result<BotOutput, BotError> {
        let bots = self.bots.read().await;
        
        // Find the best bot for this task
        let mut best_bot = None;
        let mut best_priority = 0u8;
        
        for bot in bots.values() {
            if bot.can_handle(&task.task_type).await {
                let priority = bot.get_priority(&task.task_type);
                if priority > best_priority {
                    best_priority = priority;
                    best_bot = Some(bot.clone());
                }
            }
        }
        
        match best_bot {
            Some(bot) => {
                let start_time = std::time::Instant::now();
                let mut result = bot.analyze(&task).await?;
                result.processing_time_ms = start_time.elapsed().as_millis();
                Ok(result)
            }
            None => Err(BotError::ProcessingError(
                format!("No bot available to handle task type: {}", task.task_type)
            ))
        }
    }
    
    /// Queue task for processing
    pub async fn queue_task(&self, task: BotInput) {
        self.task_queue.write().await.push(task);
    }
    
    /// Process queued tasks
    pub async fn process_queue(&self) -> Vec<Result<BotOutput, BotError>> {
        let mut queue = self.task_queue.write().await;
        let tasks = queue.drain(..).collect::<Vec<_>>();
        drop(queue);
        
        let mut results = Vec::new();
        for task in tasks {
            results.push(self.route_task(task).await);
        }
        results
    }
    
    /// Get all registered bots
    pub async fn list_bots(&self) -> Vec<(Uuid, BotSpecialty, String)> {
        let bots = self.bots.read().await;
        bots.values()
            .map(|bot| (bot.get_id(), bot.get_specialty(), bot.get_name().to_string()))
            .collect()
    }
}

/// Bot status for monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct BotStatus {
    pub bot_id: Uuid,
    pub name: String,
    pub specialty: BotSpecialty,
    pub active: bool,
    pub tasks_processed: u64,
    pub avg_processing_time_ms: f64,
    pub success_rate: f64,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Task execution context
#[derive(Debug, Clone)]
pub struct TaskContext {
    pub case_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub document_ids: Vec<Uuid>,
    pub timeline_events: Vec<Uuid>,
    pub legal_standards: Vec<String>,
    pub jurisdiction: String,
    pub urgency_level: u8,
}

impl TaskContext {
    pub fn new() -> Self {
        Self {
            case_id: None,
            client_id: None,
            document_ids: Vec::new(),
            timeline_events: Vec::new(),
            legal_standards: Vec::new(),
            jurisdiction: "Wisconsin".to_string(),
            urgency_level: 1,
        }
    }
    
    pub fn with_case(mut self, case_id: Uuid) -> Self {
        self.case_id = Some(case_id);
        self
    }
    
    pub fn with_urgency(mut self, level: u8) -> Self {
        self.urgency_level = level;
        self
    }
}

/// Bot collaboration framework
#[derive(Debug)]
pub struct BotCollaboration {
    pub collaboration_id: Uuid,
    pub primary_bot: BotSpecialty,
    pub supporting_bots: Vec<BotSpecialty>,
    pub coordination_strategy: CollaborationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationStrategy {
    Sequential,  // Bots work in sequence
    Parallel,    // Bots work simultaneously
    Hierarchical, // Lead bot coordinates others
    Consensus,   // All bots must agree
}

impl BotCollaboration {
    pub fn new(primary: BotSpecialty, strategy: CollaborationStrategy) -> Self {
        Self {
            collaboration_id: Uuid::new_v4(),
            primary_bot: primary,
            supporting_bots: Vec::new(),
            coordination_strategy: strategy,
        }
    }
    
    pub fn add_support_bot(mut self, bot: BotSpecialty) -> Self {
        self.supporting_bots.push(bot);
        self
    }
}
