//! # Bicycle Design System Demo
//!
//! This module demonstrates the usage of the bicycle design and testing system,
//! showcasing design creation, simulation, materials selection, and testing.

use bicycle::design::{BicycleDesign, BicycleType, ValidationStatus};
use bicycle::simulation::SimulationResult;
use bicycle::physics::PhysicsEngine;
use bicycle::materials::{Material, MaterialComparison, MaterialSelectionCriteria, MaterialApplication};
use bicycle::testing::{TestSuite};
use bicycle::{BicycleSystem, BicycleSystemConfig};
use std::collections::HashMap;
use uuid::Uuid;

/// Demonstrate bicycle design and testing workflow
fn main() {
    println!("🚴‍♂️ Welcome to the Bicycle Design Demo!");

    // Initialize the bicycle system
    let mut system = BicycleSystem::new(BicycleSystemConfig::default());
    
    // Create a new bicycle design
    let design_id = system.create_design("Road Racer Pro");
    let design = system.active_designs.get(&design_id).unwrap();
    
    println!("\n🎨 Created new design: {:#?}", design);

    // Simulate the bicycle design
    println!("\n🚀 Running simulations...");
    let simulation_result = system.run_simulation(&design_id).unwrap();
    println!("Simulation Result: {:#?}", simulation_result);

    // Perform material selection
    println!("\n🔬 Selecting best materials...");
    let comparison = perform_material_selection();
    println!("Best Materials: {:#?}", comparison);

    // Testing and validation
    println!("\n🧪 Running tests...");
    let test_suite = TestSuite::road_bike_standard();
    let test_result = test_suite.execute(design, &PhysicsEngine::new());
    println!("Test Result: {:#?}", test_result);

    // Run additional steps if necessary
    println!("\n✅ Demo complete!");
}

fn perform_material_selection() -> Vec<(Material, f64)> {
    let mut comparison = MaterialComparison::new(MaterialSelectionCriteria { 
        weight_importance: 0.35,
        strength_importance: 0.25,
        cost_importance: 0.2,
        environmental_importance: 0.1,
        durability_importance: 0.1,
    });

    // Evaluate common materials for a bicycle frame
    let materials = vec![
        Material::Aluminum6061,
        Material::Aluminum7075,
        Material::CarbonFiber,
        Material::Titanium,
        Material::Steel,
    ];

    for material in materials {
        comparison.evaluate_material(material, MaterialApplication::Frame);
    }

    // Return top 3 materials
    comparison.get_best_materials(3)
}
