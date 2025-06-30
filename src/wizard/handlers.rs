use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

use super::*;
use crate::db::DbPool;

pub type SharedWizardManager = Arc<Mutex<WizardManager>>;

/// Create a new wizard instance
pub async fn create_wizard(
    State(_pool): State<DbPool>,
    Json(request): Json<CreateWizardRequest>,
) -> Result<Json<WizardResponse>, StatusCode> {
    // For now, create a local wizard manager
    // In production, this would be a shared state
    let mut manager = WizardManager::new();
    manager.register_wizard(Box::new(case_wizard::CaseCreationWizard::new()));
    
    let state = manager.create_wizard(request).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // For demo purposes, use a local wizard instance
    let wizard = case_wizard::CaseCreationWizard::new();
    
    let current_step_config = wizard.get_steps().get(state.current_step)
        .cloned()
        .unwrap_or_else(|| manager.get_default_step());
    
    let navigation = manager.calculate_navigation(&state);
    
    Ok(Json(WizardResponse {
        state,
        current_step_config,
        navigation,
        errors: Vec::new(),
    }))
}

/// Get wizard state and current step
pub async fn get_wizard(
    State(_pool): State<DbPool>,
    Path(wizard_id): Path<String>,
) -> Result<Json<WizardResponse>, StatusCode> {
    // Mock implementation - in production this would fetch from database
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Submit step data and navigate
pub async fn submit_step(
    State(_pool): State<DbPool>,
    Json(submission): Json<StepSubmission>,
) -> Result<Json<WizardResponse>, StatusCode> {
    // Mock implementation - in production this would use shared wizard manager
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Get available wizard types
pub async fn get_wizard_types(
    State(_pool): State<DbPool>,
) -> Result<Json<Value>, StatusCode> {
    let wizard_types = serde_json::json!([
        {
            "type": "CaseCreation",
            "name": "Case Creation Wizard",
            "description": "Step-by-step guide to create a new legal case",
            "estimated_time": "5-10 minutes",
            "steps": 5
        },
        {
            "type": "ProjectSetup", 
            "name": "Project Setup Wizard",
            "description": "Configure a new legal project with tasks and timelines",
            "estimated_time": "10-15 minutes",
            "steps": 6
        },
        {
            "type": "ClientOnboarding",
            "name": "Client Onboarding Wizard", 
            "description": "Complete client intake and setup process",
            "estimated_time": "15-20 minutes",
            "steps": 7
        },
        {
            "type": "SalesforceIntegration",
            "name": "Salesforce Integration Wizard",
            "description": "Connect and configure Salesforce CRM integration",
            "estimated_time": "20-30 minutes", 
            "steps": 8
        }
    ]);
    
    Ok(Json(wizard_types))
}

/// Render wizard UI page
pub async fn wizard_ui() -> axum::response::Html<String> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MoodBridge Wizard Engine</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            color: #333;
            padding: 20px;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .header {
            text-align: center;
            color: white;
            margin-bottom: 40px;
        }
        .header h1 {
            font-size: 3rem;
            margin-bottom: 10px;
        }
        .header p {
            font-size: 1.2rem;
            opacity: 0.9;
        }
        .wizard-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 30px;
            margin-bottom: 40px;
        }
        .wizard-card {
            background: white;
            border-radius: 20px;
            padding: 30px;
            box-shadow: 0 15px 40px rgba(0,0,0,0.1);
            transition: transform 0.3s ease, box-shadow 0.3s ease;
            cursor: pointer;
        }
        .wizard-card:hover {
            transform: translateY(-8px);
            box-shadow: 0 25px 60px rgba(0,0,0,0.15);
        }
        .wizard-icon {
            font-size: 3rem;
            margin-bottom: 20px;
            display: block;
        }
        .wizard-title {
            font-size: 1.5rem;
            font-weight: bold;
            margin-bottom: 15px;
            color: #333;
        }
        .wizard-description {
            color: #666;
            line-height: 1.6;
            margin-bottom: 20px;
        }
        .wizard-meta {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding-top: 20px;
            border-top: 1px solid #eee;
        }
        .wizard-time {
            color: #667eea;
            font-weight: 500;
        }
        .wizard-steps {
            background: #f8f9ff;
            color: #667eea;
            padding: 5px 12px;
            border-radius: 15px;
            font-size: 0.9rem;
            font-weight: 500;
        }
        .wizard-container {
            display: none;
            background: white;
            border-radius: 20px;
            padding: 40px;
            box-shadow: 0 15px 40px rgba(0,0,0,0.1);
            margin-top: 30px;
        }
        .wizard-container.active {
            display: block;
        }
        .wizard-header {
            text-align: center;
            margin-bottom: 40px;
        }
        .wizard-progress {
            background: #f0f0f0;
            border-radius: 10px;
            height: 8px;
            margin-bottom: 30px;
            overflow: hidden;
        }
        .wizard-progress-bar {
            background: linear-gradient(90deg, #667eea, #764ba2);
            height: 100%;
            border-radius: 10px;
            transition: width 0.3s ease;
        }
        .step-container {
            margin-bottom: 30px;
        }
        .step-title {
            font-size: 1.8rem;
            font-weight: bold;
            margin-bottom: 10px;
            color: #333;
        }
        .step-description {
            color: #666;
            margin-bottom: 30px;
            line-height: 1.6;
        }
        .form-group {
            margin-bottom: 25px;
        }
        .form-label {
            display: block;
            font-weight: 600;
            margin-bottom: 8px;
            color: #333;
        }
        .form-input {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 10px;
            font-size: 1rem;
            transition: border-color 0.3s ease;
        }
        .form-input:focus {
            outline: none;
            border-color: #667eea;
        }
        .form-select {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 10px;
            font-size: 1rem;
            background: white;
            transition: border-color 0.3s ease;
        }
        .form-textarea {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e0e0e0;
            border-radius: 10px;
            font-size: 1rem;
            min-height: 120px;
            resize: vertical;
            transition: border-color 0.3s ease;
        }
        .radio-group {
            display: flex;
            gap: 20px;
            flex-wrap: wrap;
        }
        .radio-option {
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .checkbox-group {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .help-text {
            font-size: 0.9rem;
            color: #888;
            margin-top: 5px;
        }
        .wizard-actions {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-top: 40px;
            padding-top: 30px;
            border-top: 1px solid #eee;
        }
        .btn {
            padding: 12px 30px;
            border: none;
            border-radius: 10px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
        }
        .btn-primary {
            background: linear-gradient(135deg, #667eea, #764ba2);
            color: white;
        }
        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 25px rgba(102, 126, 234, 0.3);
        }
        .btn-secondary {
            background: #f8f9ff;
            color: #667eea;
            border: 2px solid #667eea;
        }
        .btn-secondary:hover {
            background: #667eea;
            color: white;
        }
        .btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none !important;
        }
        .error {
            color: #e74c3c;
            font-size: 0.9rem;
            margin-top: 5px;
        }
        .back-btn {
            background: none;
            border: none;
            color: #667eea;
            font-size: 1rem;
            cursor: pointer;
            text-decoration: underline;
        }
        .back-btn:hover {
            color: #764ba2;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🧙‍♂️ MoodBridge Wizard Engine</h1>
            <p>Streamlined workflows for legal case management</p>
        </div>
        
        <div id="wizard-selection" class="wizard-grid">
            <!-- Wizard cards will be populated by JavaScript -->
        </div>
        
        <div id="wizard-container" class="wizard-container">
            <div class="wizard-header">
                <h2 id="wizard-title">Case Creation Wizard</h2>
                <div class="wizard-progress">
                    <div id="progress-bar" class="wizard-progress-bar" style="width: 20%"></div>
                </div>
            </div>
            
            <div id="step-content">
                <!-- Step content will be populated by JavaScript -->
            </div>
            
            <div class="wizard-actions">
                <button class="back-btn" onclick="showWizardSelection()">← Back to Wizards</button>
                <div>
                    <button id="prev-btn" class="btn btn-secondary" onclick="previousStep()">Previous</button>
                    <button id="next-btn" class="btn btn-primary" onclick="nextStep()">Next</button>
                    <button id="complete-btn" class="btn btn-primary" onclick="completeWizard()" style="display: none;">Complete</button>
                </div>
            </div>
        </div>
    </div>

    <script>
        let currentWizard = null;
        let currentStep = 0;
        let wizardData = {};
        
        // Available wizards
        const wizards = [
            {
                type: 'CaseCreation',
                icon: '⚖️',
                name: 'Case Creation Wizard',
                description: 'Step-by-step guide to create a new legal case with client information, case details, and billing setup.',
                time: '5-10 minutes',
                steps: 5
            },
            {
                type: 'ProjectSetup',
                icon: '📋',
                name: 'Project Setup Wizard',
                description: 'Configure a new legal project with tasks, timelines, and team assignments.',
                time: '10-15 minutes',
                steps: 6
            },
            {
                type: 'ClientOnboarding',
                icon: '👥',
                name: 'Client Onboarding Wizard',
                description: 'Complete client intake process with contact information, preferences, and initial consultation.',
                time: '15-20 minutes',
                steps: 7
            },
            {
                type: 'SalesforceIntegration',
                icon: '🔗',
                name: 'Salesforce Integration Wizard',
                description: 'Connect and configure Salesforce CRM integration with authentication and data mapping.',
                time: '20-30 minutes',
                steps: 8
            }
        ];
        
        // Initialize the page
        function init() {
            showWizardSelection();
        }
        
        // Show wizard selection screen
        function showWizardSelection() {
            document.getElementById('wizard-selection').style.display = 'grid';
            document.getElementById('wizard-container').classList.remove('active');
            
            const wizardGrid = document.getElementById('wizard-selection');
            wizardGrid.innerHTML = wizards.map(wizard => `
                <div class="wizard-card" onclick="startWizard('${wizard.type}')">
                    <span class="wizard-icon">${wizard.icon}</span>
                    <div class="wizard-title">${wizard.name}</div>
                    <div class="wizard-description">${wizard.description}</div>
                    <div class="wizard-meta">
                        <span class="wizard-time">${wizard.time}</span>
                        <span class="wizard-steps">${wizard.steps} steps</span>
                    </div>
                </div>
            `).join('');
        }
        
        // Start a wizard
        async function startWizard(wizardType) {
            currentWizard = wizards.find(w => w.type === wizardType);
            currentStep = 0;
            wizardData = {};
            
            document.getElementById('wizard-selection').style.display = 'none';
            document.getElementById('wizard-container').classList.add('active');
            document.getElementById('wizard-title').textContent = currentWizard.name;
            
            // Create wizard via API
            try {
                const response = await fetch('/api/wizards', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ wizard_type: wizardType })
                });
                
                if (response.ok) {
                    const data = await response.json();
                    renderStep(data.current_step_config);
                    updateProgress();
                    updateNavigation(data.navigation);
                } else {
                    // Fallback to mock data
                    renderMockStep();
                }
            } catch (error) {
                console.error('Error starting wizard:', error);
                renderMockStep();
            }
        }
        
        // Render mock step for demonstration
        function renderMockStep() {
            const steps = {
                0: {
                    title: "Case Type Selection",
                    description: "Select the type of legal case you want to create",
                    fields: [
                        {
                            name: "case_type",
                            label: "Case Type",
                            field_type: "Select",
                            required: true,
                            options: [
                                { value: "family_law", label: "Family Law" },
                                { value: "criminal_defense", label: "Criminal Defense" },
                                { value: "personal_injury", label: "Personal Injury" },
                                { value: "business_law", label: "Business Law" }
                            ],
                            help_text: "Choose the primary area of law for this case"
                        }
                    ]
                },
                1: {
                    title: "Client Information",
                    description: "Enter the primary client details for this case",
                    fields: [
                        {
                            name: "client_type",
                            label: "Client Type",
                            field_type: "Radio",
                            required: true,
                            options: [
                                { value: "individual", label: "Individual" },
                                { value: "business", label: "Business/Corporation" }
                            ]
                        },
                        {
                            name: "client_first_name",
                            label: "First Name",
                            field_type: "Text",
                            required: true
                        },
                        {
                            name: "client_email",
                            label: "Email Address",
                            field_type: "Email",
                            required: true,
                            help_text: "Primary contact email for case communications"
                        }
                    ]
                }
            };
            
            const step = steps[currentStep] || steps[0];
            renderStep(step);
            updateProgress();
            updateNavigation({ can_go_previous: currentStep > 0, can_go_next: true, can_complete: currentStep >= 4 });
        }
        
        // Render step content
        function renderStep(step) {
            const content = document.getElementById('step-content');
            content.innerHTML = `
                <div class="step-container">
                    <h3 class="step-title">${step.title}</h3>
                    <p class="step-description">${step.description}</p>
                    ${step.fields.map(field => renderField(field)).join('')}
                </div>
            `;
        }
        
        // Render individual form field
        function renderField(field) {
            const value = wizardData[field.name] || field.default_value || '';
            
            switch (field.field_type) {
                case 'Text':
                case 'Email':
                    return `
                        <div class="form-group">
                            <label class="form-label">${field.label}${field.required ? ' *' : ''}</label>
                            <input type="${field.field_type.toLowerCase()}" 
                                   class="form-input" 
                                   name="${field.name}"
                                   value="${value}"
                                   ${field.required ? 'required' : ''}
                                   onchange="updateWizardData('${field.name}', this.value)">
                            ${field.help_text ? `<div class="help-text">${field.help_text}</div>` : ''}
                        </div>
                    `;
                
                case 'TextArea':
                    return `
                        <div class="form-group">
                            <label class="form-label">${field.label}${field.required ? ' *' : ''}</label>
                            <textarea class="form-textarea" 
                                      name="${field.name}"
                                      ${field.required ? 'required' : ''}
                                      onchange="updateWizardData('${field.name}', this.value)">${value}</textarea>
                            ${field.help_text ? `<div class="help-text">${field.help_text}</div>` : ''}
                        </div>
                    `;
                
                case 'Select':
                    return `
                        <div class="form-group">
                            <label class="form-label">${field.label}${field.required ? ' *' : ''}</label>
                            <select class="form-select" 
                                    name="${field.name}"
                                    ${field.required ? 'required' : ''}
                                    onchange="updateWizardData('${field.name}', this.value)">
                                <option value="">Select an option...</option>
                                ${field.options ? field.options.map(opt => 
                                    `<option value="${opt.value}" ${value === opt.value ? 'selected' : ''}>${opt.label}</option>`
                                ).join('') : ''}
                            </select>
                            ${field.help_text ? `<div class="help-text">${field.help_text}</div>` : ''}
                        </div>
                    `;
                
                case 'Radio':
                    return `
                        <div class="form-group">
                            <label class="form-label">${field.label}${field.required ? ' *' : ''}</label>
                            <div class="radio-group">
                                ${field.options ? field.options.map(opt => `
                                    <div class="radio-option">
                                        <input type="radio" 
                                               name="${field.name}" 
                                               value="${opt.value}"
                                               ${value === opt.value ? 'checked' : ''}
                                               onchange="updateWizardData('${field.name}', this.value)">
                                        <label>${opt.label}</label>
                                    </div>
                                `).join('') : ''}
                            </div>
                            ${field.help_text ? `<div class="help-text">${field.help_text}</div>` : ''}
                        </div>
                    `;
                
                default:
                    return `<div class="form-group">Unsupported field type: ${field.field_type}</div>`;
            }
        }
        
        // Update wizard data
        function updateWizardData(field, value) {
            wizardData[field] = value;
            console.log('Updated wizard data:', wizardData);
        }
        
        // Update progress bar
        function updateProgress() {
            if (currentWizard) {
                const progress = ((currentStep + 1) / currentWizard.steps) * 100;
                document.getElementById('progress-bar').style.width = progress + '%';
            }
        }
        
        // Update navigation buttons
        function updateNavigation(nav) {
            document.getElementById('prev-btn').disabled = !nav.can_go_previous;
            document.getElementById('next-btn').style.display = nav.can_complete ? 'none' : 'inline-block';
            document.getElementById('complete-btn').style.display = nav.can_complete ? 'inline-block' : 'none';
        }
        
        // Navigate to previous step
        function previousStep() {
            if (currentStep > 0) {
                currentStep--;
                renderMockStep();
            }
        }
        
        // Navigate to next step
        function nextStep() {
            if (currentWizard && currentStep < currentWizard.steps - 1) {
                currentStep++;
                renderMockStep();
            }
        }
        
        // Complete wizard
        function completeWizard() {
            alert('Wizard completed successfully! \\n\\nCollected data: ' + JSON.stringify(wizardData, null, 2));
            showWizardSelection();
        }
        
        // Initialize when page loads
        document.addEventListener('DOMContentLoaded', init);
    </script>
</body>
</html>
    "#;
    
    axum::response::Html(html.to_string())
}
