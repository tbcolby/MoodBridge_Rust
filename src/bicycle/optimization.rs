//! # Optimization Module
//!
//! This module provides optimization algorithms for bicycle design,
//! including genetic algorithms, simulated annealing, and multi-objective optimization.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use super::design::BicycleDesign;
use super::simulation::SimulationResult;

/// Optimization objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationObjective {
    MinimizeWeight,
    MaximizeSpeed,
    MinimizeCost,
    MaximizeComfort,
    MaximizeSafety,
    MinimizeEnvironmentalImpact,
    MultiObjective(Vec<(OptimizationObjective, f64)>), // Objectives with weights
}

/// Optimization constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConstraints {
    pub max_weight: Option<f64>,
    pub max_cost: Option<f64>,
    pub min_safety_score: Option<f64>,
    pub min_comfort_rating: Option<f64>,
    pub required_certifications: Vec<String>,
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_design_id: Uuid,
    pub optimized_design: BicycleDesign,
    pub improvement_metrics: HashMap<String, f64>,
    pub optimization_iterations: u32,
    pub convergence_achieved: bool,
    pub objective_value: f64,
}

/// Genetic algorithm optimizer
pub struct GeneticOptimizer {
    pub population_size: u32,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub max_generations: u32,
    pub elitism_rate: f64,
}

impl Default for GeneticOptimizer {
    fn default() -> Self {
        Self {
            population_size: 50,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            max_generations: 100,
            elitism_rate: 0.1,
        }
    }
}

impl GeneticOptimizer {
    pub fn optimize(
        &self,
        base_design: &BicycleDesign,
        objective: OptimizationObjective,
        constraints: OptimizationConstraints,
    ) -> OptimizationResult {
        // Simplified genetic algorithm implementation
        let mut best_design = base_design.clone();
        let mut best_fitness = self.evaluate_fitness(&best_design, &objective);
        
        for generation in 0..self.max_generations {
            // Create population variations
            let population = self.create_population(&best_design);
            
            // Evaluate fitness for each design
            let mut fitness_scores = Vec::new();
            for design in &population {
                let fitness = self.evaluate_fitness(design, &objective);
                fitness_scores.push(fitness);
                
                if fitness > best_fitness {
                    best_fitness = fitness;
                    best_design = design.clone();
                }
            }
            
            // Check convergence
            if generation > 10 && self.check_convergence(&fitness_scores) {
                break;
            }
        }
        
        let improvement_metrics = self.calculate_improvements(base_design, &best_design);
        
        OptimizationResult {
            original_design_id: base_design.id,
            optimized_design: best_design,
            improvement_metrics,
            optimization_iterations: self.max_generations,
            convergence_achieved: true,
            objective_value: best_fitness,
        }
    }
    
    fn create_population(&self, base_design: &BicycleDesign) -> Vec<BicycleDesign> {
        let mut population = Vec::new();
        
        for _ in 0..self.population_size {
            let mut variant = base_design.clone();
            variant.id = Uuid::new_v4();
            
            // Apply random mutations
            self.mutate_design(&mut variant);
            population.push(variant);
        }
        
        population
    }
    
    fn mutate_design(&self, design: &mut BicycleDesign) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen::<f64>() < self.mutation_rate {
            // Mutate frame geometry
            design.frame.geometry.seat_tube_length *= 1.0 + (rng.gen::<f64>() - 0.5) * 0.1;
            design.frame.geometry.top_tube_length *= 1.0 + (rng.gen::<f64>() - 0.5) * 0.1;
        }
        
        if rng.gen::<f64>() < self.mutation_rate {
            // Mutate target weight
            design.target_weight *= 1.0 + (rng.gen::<f64>() - 0.5) * 0.2;
        }
        
        if rng.gen::<f64>() < self.mutation_rate {
            // Mutate wheel specifications
            design.wheels.front_wheel.weight *= 1.0 + (rng.gen::<f64>() - 0.5) * 0.15;
            design.wheels.rear_wheel.weight *= 1.0 + (rng.gen::<f64>() - 0.5) * 0.15;
        }
    }
    
    fn evaluate_fitness(&self, design: &BicycleDesign, objective: &OptimizationObjective) -> f64 {
        match objective {
            OptimizationObjective::MinimizeWeight => {
                let weight = design.calculate_total_weight();
                1.0 / (weight + 1.0) * 100.0 // Higher fitness for lower weight
            },
            OptimizationObjective::MaximizeSpeed => {
                // Simplified speed calculation based on weight and aerodynamics
                let weight = design.calculate_total_weight();
                let aero_factor = match design.intended_use {
                    super::design::BicycleType::Road => 1.2,
                    _ => 1.0,
                };
                (25.0 - weight / 2.0) * aero_factor
            },
            OptimizationObjective::MinimizeCost => {
                1.0 / (design.target_price + 1.0) * 10000.0
            },
            OptimizationObjective::MaximizeComfort => {
                // Simplified comfort based on frame stiffness
                10.0 - (design.frame.stiffness_rating / 15.0)
            },
            OptimizationObjective::MaximizeSafety => {
                design.frame.fatigue_life as f64 / 100000.0 * 100.0
            },
            OptimizationObjective::MinimizeEnvironmentalImpact => {
                design.frame.material.environmental_score()
            },
            OptimizationObjective::MultiObjective(objectives) => {
                let mut total_score = 0.0;
                let mut total_weight = 0.0;
                
                for (obj, weight) in objectives {
                    let score = self.evaluate_fitness(design, obj);
                    total_score += score * weight;
                    total_weight += weight;
                }
                
                if total_weight > 0.0 {
                    total_score / total_weight
                } else {
                    0.0
                }
            },
        }
    }
    
    fn check_convergence(&self, fitness_scores: &[f64]) -> bool {
        if fitness_scores.len() < 10 {
            return false;
        }
        
        let recent_scores = &fitness_scores[fitness_scores.len() - 10..];
        let max_score = recent_scores.iter().fold(0.0_f64, |a, &b| a.max(b));
        let min_score = recent_scores.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        (max_score - min_score) < 0.01 // Convergence threshold
    }
    
    fn calculate_improvements(&self, original: &BicycleDesign, optimized: &BicycleDesign) -> HashMap<String, f64> {
        let mut improvements = HashMap::new();
        
        let weight_improvement = (original.calculate_total_weight() - optimized.calculate_total_weight()) 
            / original.calculate_total_weight() * 100.0;
        improvements.insert("weight_reduction_percent".to_string(), weight_improvement);
        
        let cost_improvement = (original.target_price - optimized.target_price) 
            / original.target_price * 100.0;
        improvements.insert("cost_reduction_percent".to_string(), cost_improvement);
        
        improvements
    }
}

/// Simulated annealing optimizer
pub struct SimulatedAnnealingOptimizer {
    pub initial_temperature: f64,
    pub cooling_rate: f64,
    pub min_temperature: f64,
    pub max_iterations: u32,
}

impl Default for SimulatedAnnealingOptimizer {
    fn default() -> Self {
        Self {
            initial_temperature: 100.0,
            cooling_rate: 0.95,
            min_temperature: 0.1,
            max_iterations: 1000,
        }
    }
}

impl SimulatedAnnealingOptimizer {
    pub fn optimize(
        &self,
        base_design: &BicycleDesign,
        objective: OptimizationObjective,
        constraints: OptimizationConstraints,
    ) -> OptimizationResult {
        let mut current_design = base_design.clone();
        let mut best_design = base_design.clone();
        let mut temperature = self.initial_temperature;
        
        let genetic_optimizer = GeneticOptimizer::default();
        let mut current_fitness = genetic_optimizer.evaluate_fitness(&current_design, &objective);
        let mut best_fitness = current_fitness;
        
        for iteration in 0..self.max_iterations {
            if temperature < self.min_temperature {
                break;
            }
            
            // Create a neighbor solution
            let mut neighbor = current_design.clone();
            neighbor.id = Uuid::new_v4();
            genetic_optimizer.mutate_design(&mut neighbor);
            
            let neighbor_fitness = genetic_optimizer.evaluate_fitness(&neighbor, &objective);
            
            // Accept or reject the neighbor
            if self.should_accept(current_fitness, neighbor_fitness, temperature) {
                current_design = neighbor;
                current_fitness = neighbor_fitness;
                
                if neighbor_fitness > best_fitness {
                    best_design = current_design.clone();
                    best_fitness = neighbor_fitness;
                }
            }
            
            // Cool down
            temperature *= self.cooling_rate;
        }
        
        let improvement_metrics = genetic_optimizer.calculate_improvements(base_design, &best_design);
        
        OptimizationResult {
            original_design_id: base_design.id,
            optimized_design: best_design,
            improvement_metrics,
            optimization_iterations: self.max_iterations,
            convergence_achieved: temperature < self.min_temperature,
            objective_value: best_fitness,
        }
    }
    
    fn should_accept(&self, current_fitness: f64, new_fitness: f64, temperature: f64) -> bool {
        if new_fitness > current_fitness {
            true
        } else {
            let probability = ((new_fitness - current_fitness) / temperature).exp();
            rand::random::<f64>() < probability
        }
    }
}
