use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookProject {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String,
    pub writing_style: WritingStyle,
    pub target_length: u32, // words
    pub current_word_count: u32,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: ProjectMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub genre: String,
    pub target_audience: Vec<String>,
    pub philosophical_themes: Vec<String>,
    pub technical_complexity: TechnicalComplexity,
    pub collaboration_model: CollaborationModel,
    pub publication_format: Vec<PublicationFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WritingStyle {
    Hofstadterian {
        recursive_depth: u8,
        analogy_density: f32,
        mathematical_rigor: bool,
        self_referential: bool,
    },
    Academic {
        citation_style: String,
        formality_level: u8,
        technical_depth: u8,
    },
    Narrative {
        voice: String,
        tense: String,
        pov: String,
    },
    Hybrid {
        primary_style: Box<WritingStyle>,
        secondary_elements: Vec<WritingStyle>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnicalComplexity {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
    ResearchLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    Outlining,
    Writing,
    Editing,
    Review,
    Finalizing,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationModel {
    SingleAuthor,
    AIAssisted,
    MultiAgent {
        lead_agent: String,
        supporting_agents: Vec<String>,
    },
    HumanAICollaborative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicationFormat {
    DigitalInteractive,
    Print,
    Ebook,
    AudioBook,
    WebSerial,
    Academic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: Uuid,
    pub project_id: Uuid,
    pub chapter_number: u32,
    pub title: String,
    pub content: String,
    pub word_count: u32,
    pub status: ChapterStatus,
    pub assigned_agent: Option<Uuid>,
    pub narrative_threads: Vec<String>,
    pub philosophical_elements: Vec<PhilosophicalElement>,
    pub technical_concepts: Vec<TechnicalConcept>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChapterStatus {
    Planned,
    Outlined,
    Drafting,
    FirstDraft,
    Reviewing,
    Editing,
    Finalized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalElement {
    pub concept: String,
    pub explanation: String,
    pub connections: Vec<String>,
    pub depth_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalConcept {
    pub name: String,
    pub domain: String,
    pub complexity: TechnicalComplexity,
    pub explanation: String,
    pub code_examples: Vec<CodeExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub code: String,
    pub explanation: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGenerationRequest {
    pub project_id: Uuid,
    pub content_type: ContentType,
    pub prompt: String,
    pub style_preferences: WritingStyle,
    pub length_target: Option<u32>,
    pub context: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    ChapterDraft,
    SectionExpansion,
    PhilosophicalAnalysis,
    TechnicalExplanation,
    CodeDocumentation,
    CrossReference,
    Conclusion,
    Introduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardSession {
    pub id: Uuid,
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub collected_data: HashMap<String, serde_json::Value>,
    pub project_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookOutline {
    pub project_id: Uuid,
    pub structure: OutlineStructure,
    pub narrative_arc: NarrativeArc,
    pub philosophical_journey: PhilosophicalJourney,
    pub technical_progression: TechnicalProgression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineStructure {
    pub total_chapters: u32,
    pub chapter_outlines: Vec<ChapterOutline>,
    pub appendices: Vec<AppendixOutline>,
    pub cross_references: Vec<CrossReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterOutline {
    pub number: u32,
    pub title: String,
    pub main_theme: String,
    pub philosophical_focus: Vec<String>,
    pub technical_topics: Vec<String>,
    pub narrative_threads: Vec<String>,
    pub estimated_word_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeArc {
    pub opening_hook: String,
    pub rising_action: Vec<String>,
    pub climax: String,
    pub resolution: String,
    pub themes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalJourney {
    pub central_question: String,
    pub supporting_questions: Vec<String>,
    pub paradoxes_explored: Vec<String>,
    pub resolution_approach: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalProgression {
    pub starting_complexity: TechnicalComplexity,
    pub ending_complexity: TechnicalComplexity,
    pub key_concepts: Vec<String>,
    pub implementation_examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendixOutline {
    pub title: String,
    pub content_type: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub from_chapter: u32,
    pub to_chapter: u32,
    pub concept: String,
    pub reference_type: CrossReferenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossReferenceType {
    ConceptualConnection,
    TechnicalDependency,
    PhilosophicalParallel,
    NarrativeContinuation,
    StrangeLoop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveElement {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub element_type: InteractiveElementType,
    pub title: String,
    pub description: String,
    pub implementation: String, // HTML/JS/CSS
    pub educational_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractiveElementType {
    CodePlayground,
    PhilosophicalThoughtExperiment,
    ConceptualDiagram,
    DecisionTree,
    ParametricVisualization,
    StrangeLoopExplorer,
}

impl Default for BookProject {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            author: String::new(),
            writing_style: WritingStyle::Hofstadterian {
                recursive_depth: 3,
                analogy_density: 0.7,
                mathematical_rigor: true,
                self_referential: true,
            },
            target_length: 200_000, // 600+ pages
            current_word_count: 0,
            status: ProjectStatus::Planning,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: ProjectMetadata::default(),
        }
    }
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            genre: "Technical Philosophy".to_string(),
            target_audience: vec![
                "Technical Professionals".to_string(),
                "Legal Experts".to_string(),
                "AI Researchers".to_string(),
                "Philosophy Enthusiasts".to_string(),
            ],
            philosophical_themes: vec![
                "Consciousness and AI".to_string(),
                "Strange Loops".to_string(),
                "Legal Reasoning".to_string(),
                "Human-Machine Collaboration".to_string(),
            ],
            technical_complexity: TechnicalComplexity::Advanced,
            collaboration_model: CollaborationModel::MultiAgent {
                lead_agent: "HofstadterianWriter".to_string(),
                supporting_agents: vec![
                    "TechnicalDocumenter".to_string(),
                    "PhilosophicalAnalyzer".to_string(),
                    "LegalExpert".to_string(),
                ],
            },
            publication_format: vec![
                PublicationFormat::DigitalInteractive,
                PublicationFormat::Print,
            ],
        }
    }
}
