//! # Bicycle Design System Demo
//!
//! A comprehensive demonstration of the bicycle design and testing capabilities.

use moodbridge_rust::bicycle::{
    BicycleSystem, BicycleSystemConfig, SimulationPrecision, PhysicsEngine,
    design::{BicycleDesign, BicycleType, ValidationStatus},
    materials::{Material, MaterialComparison, MaterialSelectionCriteria, MaterialApplication},
    testing::TestSuite,
    optimization::{GeneticOptimizer, OptimizationObjective, OptimizationConstraints},
    aerodynamics::AerodynamicAnalyzer,
    wizard::{BicycleWizard, WizardAnswer, AnswerValue},
};
use std::collections::HashMap;

fn main() {
    println!("🚴‍♂️ BICYCLE DESIGN SYSTEM DEMO");
    println!("═══════════════════════════════════════");
    
    // Initialize the system
    let config = BicycleSystemConfig {
        simulation_precision: SimulationPrecision::High,
        ..Default::default()
    };
    let mut system = BicycleSystem::new(config);
    
    // Demo 1: Create and analyze a road bike design
    demo_road_bike_design(&mut system);
    
    // Demo 2: Material selection and comparison
    demo_material_selection();
    
    // Demo 3: Testing and certification
    demo_testing_and_certification();
    
    // Demo 4: Optimization
    demo_optimization(&mut system);
    
    // Demo 5: Aerodynamic analysis
    demo_aerodynamics();
    
    // Demo 6: Design wizard
    demo_design_wizard();
    
    println!("\n✅ Demo completed successfully!");
    println!("   Visit http://127.0.0.1:8000/api/bicycle/health to access the REST API");
}

fn demo_road_bike_design(system: &mut BicycleSystem) {
    println!("\n🎨 DEMO 1: ROAD BIKE DESIGN & SIMULATION");
    println!("──────────────────────────────────────────");
    
    // Create a new road bike design
    let design_id = system.create_design("Aero Road Racer Pro");
    let design = system.active_designs.get_mut(&design_id).unwrap();
    
    // Customize the design
    design.intended_use = BicycleType::Road;
    design.target_weight = 7.2;
    design.target_price = 4500.0;
    design.frame.material = Material::CarbonFiber;
    
    println!("📋 Created design: {}", design.name);
    println!("   Type: {:?}", design.intended_use);
    println!("   Target weight: {:.1} kg", design.target_weight);
    println!("   Target price: ${:.0}", design.target_price);
    println!("   Frame material: {:?}", design.frame.material);
    
    // Validate the design
    let validation_issues = design.validate_design();
    if validation_issues.is_empty() {
        println!("✅ Design validation passed");
        design.validation_status = ValidationStatus::Validated;
    } else {
        println!("⚠️  Design validation issues:");
        for issue in validation_issues {
            println!("   - {}", issue);
        }
    }
    
    // Calculate metrics
    let total_weight = design.calculate_total_weight();
    let (min_height, max_height) = design.get_rider_size_range();
    
    println!("📊 Design Metrics:");
    println!("   Total weight: {:.1} kg", total_weight);
    println!("   Rider height range: {:.0}-{:.0} cm", min_height, max_height);
    
    // Run simulation
    println!("\n🚀 Running performance simulation...");
    match system.run_simulation(&design_id) {
        Ok(result) => {
            println!("   Speed: {:.1} km/h", result.speed);
            println!("   Acceleration: {:.1} m/s²", result.acceleration);
            println!("   Stopping distance: {:.1} m", result.stopping_distance);
            println!("   Comfort rating: {:.1}/10", result.comfort_rating);
            println!("   Safety score: {:.1}/100", result.safety_score);
        },
        Err(e) => println!("❌ Simulation failed: {}", e),
    }
}

fn demo_material_selection() {
    println!("\n🔬 DEMO 2: MATERIAL SELECTION & COMPARISON");
    println!("─────────────────────────────────────────────");
    
    let criteria = MaterialSelectionCriteria {
        weight_importance: 0.35,
        strength_importance: 0.25,
        cost_importance: 0.20,
        environmental_importance: 0.10,
        durability_importance: 0.10,
    };
    
    let mut comparison = MaterialComparison::new(criteria);
    
    // Evaluate materials for frame application
    let materials = vec![
        Material::Steel,
        Material::Aluminum6061,
        Material::Aluminum7075,
        Material::CarbonFiber,
        Material::Titanium,
    ];
    
    println!("🏗️  Evaluating frame materials...");
    for material in &materials {
        comparison.evaluate_material(material.clone(), MaterialApplication::Frame);
        let props = material.properties();
        let strength_ratio = material.strength_to_weight_ratio();
        
        println!("   {:?}:", material);
        println!("     Density: {:.0} kg/m³", props.density);
        println!("     Strength/Weight: {:.1}", strength_ratio);
        println!("     Cost: ${:.1}/kg", props.cost_per_kg);
        println!("     Environmental score: {:.1}/100", material.environmental_score());
    }
    
    // Get best materials
    let best_materials = comparison.get_best_materials(3);
    println!("\n🏆 Top 3 materials for frame application:");
    for (i, (material, score)) in best_materials.iter().enumerate() {
        println!("   {}. {:?} (Score: {:.1})", i + 1, material, score);
    }
}

fn demo_testing_and_certification() {
    println!("\n🧪 DEMO 3: TESTING & CERTIFICATION");
    println!("──────────────────────────────────────");
    
    let design = BicycleDesign::new("Test Subject Road Bike");
    let physics_engine = PhysicsEngine::new();
    
    // Run road bike test suite
    println!("🔍 Running road bike test suite...");
    let test_suite = TestSuite::road_bike_standard();
    let test_result = test_suite.execute(&design, &physics_engine);
    
    println!("📊 Test Results:");
    println!("   Overall score: {:.1}/100", test_result.overall_score);
    println!("   Passed: {}", if test_result.passed { "✅ YES" } else { "❌ NO" });
    println!("   Total tests: {}", test_result.individual_results.len());
    
    // Show individual test results
    println!("\n📋 Individual Test Results:");
    for result in &test_result.individual_results {
        let status = if result.passed { "✅" } else { "❌" };
        println!("   {} {} (Score: {:.1})", status, result.test_name, result.score);
        if !result.passed {
            for note in &result.notes {
                println!("      💡 {}", note);
            }
        }
    }
    
    // Show certification status
    println!("\n🏅 Certification Status:");
    for (standard, passed) in &test_result.certification_status {
        let status = if *passed { "✅ PASSED" } else { "❌ FAILED" };
        println!("   {:?}: {}", standard, status);
    }
    
    // Show recommendations
    if !test_result.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for rec in &test_result.recommendations {
            println!("   - {}", rec);
        }
    }
}

fn demo_optimization(system: &mut BicycleSystem) {
    println!("\n⚡ DEMO 4: DESIGN OPTIMIZATION");
    println!("─────────────────────────────────────");
    
    // Create base design
    let design_id = system.create_design("Base Design for Optimization");
    let base_design = system.active_designs.get(&design_id).unwrap().clone();
    
    println!("🎯 Original design weight: {:.1} kg", base_design.calculate_total_weight());
    
    // Setup optimization
    let optimizer = GeneticOptimizer::default();
    let objective = OptimizationObjective::MinimizeWeight;
    let constraints = OptimizationConstraints {
        max_weight: Some(8.0),
        max_cost: Some(5000.0),
        min_safety_score: Some(80.0),
        min_comfort_rating: Some(6.0),
        required_certifications: vec!["ISO4210".to_string()],
    };
    
    println!("🔄 Running genetic algorithm optimization...");
    println!("   Objective: Minimize weight");
    println!("   Max weight constraint: 8.0 kg");
    println!("   Max cost constraint: $5000");
    
    let optimization_result = optimizer.optimize(&base_design, objective, constraints);
    
    println!("\n📈 Optimization Results:");
    println!("   Iterations: {}", optimization_result.optimization_iterations);
    println!("   Converged: {}", optimization_result.convergence_achieved);
    println!("   Objective value: {:.2}", optimization_result.objective_value);
    
    let optimized_weight = optimization_result.optimized_design.calculate_total_weight();
    println!("   Optimized weight: {:.1} kg", optimized_weight);
    
    println!("\n📊 Improvements:");
    for (metric, improvement) in &optimization_result.improvement_metrics {
        println!("   {}: {:.1}%", metric, improvement);
    }
}

fn demo_aerodynamics() {
    println!("\n💨 DEMO 5: AERODYNAMIC ANALYSIS");
    println!("───────────────────────────────────");
    
    let mut design = BicycleDesign::new("Aero Test Bike");
    design.intended_use = BicycleType::Road;
    
    let analyzer = AerodynamicAnalyzer::default();
    
    println!("🌪️  Running wind tunnel simulation...");
    let aero_analysis = analyzer.analyze(&design);
    
    println!("📊 Aerodynamic Results:");
    println!("   Drag coefficient (Cd): {:.3}", aero_analysis.drag_coefficient);
    println!("   Frontal area: {:.2} m²", aero_analysis.frontal_area);
    println!("   Drag force at 40 km/h: {:.1} N", aero_analysis.drag_force_at_40kmh);
    println!("   Power required at 40 km/h: {:.1} W", aero_analysis.power_required_at_40kmh);
    
    // Speed analysis
    let speeds = vec![8.33, 11.11, 13.89, 16.67]; // 30, 40, 50, 60 km/h in m/s
    let speed_analysis = analyzer.speed_analysis(&design, speeds);
    
    println!("\n🏃 Speed vs Power Analysis:");
    for (speed_kmh, drag_force, power) in speed_analysis {
        println!("   {:.0} km/h: {:.1} N drag, {:.0} W power", speed_kmh, drag_force, power);
    }
    
    // Show recommendations
    if !aero_analysis.recommendations.is_empty() {
        println!("\n💡 Aerodynamic Recommendations:");
        for rec in &aero_analysis.recommendations {
            println!("   - {}", rec);
        }
    }
}

fn demo_design_wizard() {
    println!("\n🧙‍♂️ DEMO 6: DESIGN WIZARD");
    println!("─────────────────────────────");
    
    let wizard = BicycleWizard::new();
    let mut session = wizard.start_session();
    
    println!("🎯 Starting guided bicycle design wizard...");
    println!("   Session ID: {}", session.session_id);
    
    // Simulate wizard progression
    let steps = vec![
        ("basic_info", vec![
            ("design_name", AnswerValue::Text("My Dream Bike".to_string())),
            ("bicycle_type", AnswerValue::SingleChoice("Road".to_string())),
        ]),
        ("rider_fit", vec![
            ("rider_height", AnswerValue::Number(175.0)),
            ("inseam", AnswerValue::Number(84.0)),
            ("flexibility", AnswerValue::SingleChoice("Medium".to_string())),
        ]),
        ("performance_goals", vec![
            ("target_weight", AnswerValue::Number(7.5)),
            ("budget", AnswerValue::SingleChoice("high_end".to_string())),
            ("priorities", AnswerValue::MultipleChoice(vec!["speed".to_string(), "weight".to_string()])),
        ]),
        ("material_preferences", vec![
            ("frame_material", AnswerValue::SingleChoice("carbon_fiber".to_string())),
        ]),
    ];
    
    for (step_name, answers) in steps {
        println!("\n📝 Step: {}", step_name);
        let step = wizard.get_step(step_name).unwrap();
        println!("   {}", step.description);
        
        let wizard_answers: Vec<WizardAnswer> = answers.into_iter()
            .map(|(id, answer)| WizardAnswer {
                question_id: id.to_string(),
                answer,
            })
            .collect();
        
        match wizard.submit_step(&mut session, wizard_answers) {
            Ok(next_step) => {
                if let Some(next) = next_step {
                    println!("   ✅ Moving to next step: {}", next);
                } else {
                    println!("   🎉 Wizard completed!");
                }
            },
            Err(e) => println!("   ❌ Error: {}", e),
        }
        
        let progress = wizard.get_progress(&session);
        println!("   Progress: {:.0}%", progress);
    }
    
    // Show generated design
    if let Some(design) = &session.generated_design {
        println!("\n🎨 Generated Design:");
        println!("   Name: {}", design.name);
        println!("   Type: {:?}", design.intended_use);
        println!("   Target weight: {:.1} kg", design.target_weight);
        println!("   Target price: ${:.0}", design.target_price);
        println!("   Frame material: {:?}", design.frame.material);
        println!("   Rider height: {:.0} cm", design.rider_fit.rider_height);
        println!("   Total weight: {:.1} kg", design.calculate_total_weight());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_initialization() {
        let system = BicycleSystem::new(BicycleSystemConfig::default());
        assert_eq!(system.active_designs.len(), 0);
    }

    #[test]
    fn test_design_creation() {
        let mut system = BicycleSystem::new(BicycleSystemConfig::default());
        let design_id = system.create_design("Test Bike");
        assert!(system.active_designs.contains_key(&design_id));
    }

    #[test]
    fn test_material_comparison() {
        let criteria = MaterialSelectionCriteria {
            weight_importance: 0.5,
            strength_importance: 0.3,
            cost_importance: 0.2,
            environmental_importance: 0.0,
            durability_importance: 0.0,
        };
        
        let mut comparison = MaterialComparison::new(criteria);
        comparison.evaluate_material(Material::CarbonFiber, MaterialApplication::Frame);
        comparison.evaluate_material(Material::Aluminum6061, MaterialApplication::Frame);
        
        let results = comparison.get_best_materials(2);
        assert_eq!(results.len(), 2);
    }
}
