//! # Bicycle Design and Testing System
//! 
//! A comprehensive bicycle design, simulation, and testing platform built on top of MoodBridge_Rust.
//! This system provides end-to-end capabilities for designing, analyzing, and testing bicycle designs
//! with real-time physics simulation, performance analysis, and optimization.

pub mod design;
pub mod physics;
pub mod simulation;
pub mod testing;
pub mod optimization;
pub mod materials;
pub mod aerodynamics;
pub mod handlers;
pub mod wizard;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core bicycle design system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleSystemConfig {
    pub simulation_precision: SimulationPrecision,
    pub physics_engine: PhysicsEngine,
    pub testing_framework: TestingFramework,
    pub optimization_algorithm: OptimizationAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationPrecision {
    Low,    // Fast prototyping
    Medium, // Balanced
    High,   // Detailed analysis
    Ultra,  // Research-grade
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicsEngine {
    Basic,      // Simple physics for quick iterations
    Advanced,   // Comprehensive physics simulation
    Quantum,    // Quantum mechanical effects (experimental)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestingFramework {
    Virtual,    // Pure simulation
    Hybrid,     // Simulation + real-world data
    Physical,   // Physical prototyping
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    GeneticAlgorithm,
    SimulatedAnnealing,
    ParticleSwarm,
    MachineLearning,
}

impl Default for BicycleSystemConfig {
    fn default() -> Self {
        Self {
            simulation_precision: SimulationPrecision::Medium,
            physics_engine: PhysicsEngine::Advanced,
            testing_framework: TestingFramework::Virtual,
            optimization_algorithm: OptimizationAlgorithm::GeneticAlgorithm,
        }
    }
}

/// Main bicycle system manager
pub struct BicycleSystem {
    pub config: BicycleSystemConfig,
    pub active_designs: HashMap<Uuid, design::BicycleDesign>,
    pub simulation_cache: HashMap<Uuid, simulation::SimulationResult>,
}

impl BicycleSystem {
    pub fn new(config: BicycleSystemConfig) -> Self {
        Self {
            config,
            active_designs: HashMap::new(),
            simulation_cache: HashMap::new(),
        }
    }

    pub fn create_design(&mut self, name: &str) -> Uuid {
        let design_id = Uuid::new_v4();
        let design = design::BicycleDesign::new(name);
        self.active_designs.insert(design_id, design);
        design_id
    }

    pub fn run_simulation(&mut self, design_id: &Uuid) -> Result<simulation::SimulationResult, String> {
        let design = self.active_designs.get(design_id)
            .ok_or("Design not found")?;
        
        let result = simulation::run_comprehensive_simulation(design, &self.config)?;
        self.simulation_cache.insert(*design_id, result.clone());
        
        Ok(result)
    }
}
