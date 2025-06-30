//! # Materials Database
//!
//! This module defines material properties and characteristics for bicycle components,
//! including mechanical properties, cost analysis, and environmental impact.

use serde::{Deserialize, Serialize};

/// Material types used in bicycle construction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Material {
    // Metals
    Steel,
    StainlessSteel,
    Aluminum6061,
    Aluminum7075,
    Titanium,
    Magnesium,
    
    // Carbon composites
    CarbonFiber,
    CarbonFiberHighModulus,
    CarbonFiberUltraLight,
    
    // Other composites
    Fiberglass,
    Kevlar,
    Basalt,
    
    // Polymers
    Nylon,
    Polyethylene,
    Polyurethane,
    
    // Specialty materials
    Scandium,
    Beryllium,
    Bamboo,
    Wood(WoodType),
    
    // Composites
    AluminumCarbon,
    SteelCarbon,
    TitaniumCarbon,
    
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WoodType {
    Ash,
    Walnut,
    Bamboo,
    Birch,
    Oak,
}

/// Material properties for engineering calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub density: f64,           // kg/m³
    pub tensile_strength: f64,  // MPa
    pub yield_strength: f64,    // MPa
    pub modulus_of_elasticity: f64, // GPa
    pub fatigue_limit: f64,     // MPa
    pub thermal_expansion: f64,  // μm/m·K
    pub corrosion_resistance: CorrosionResistance,
    pub workability: Workability,
    pub cost_per_kg: f64,       // USD/kg
    pub environmental_impact: EnvironmentalImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrosionResistance {
    Excellent,
    Good,
    Fair,
    Poor,
    RequiresProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workability {
    pub machinability: f64,     // 1-10 scale
    pub weldability: f64,       // 1-10 scale
    pub formability: f64,       // 1-10 scale
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub recyclability: f64,     // 0-1 scale
    pub carbon_footprint: f64,  // kg CO2 equivalent per kg
    pub energy_to_produce: f64, // MJ/kg
    pub biodegradability: f64,  // 0-1 scale
}

impl Material {
    /// Get the material properties for engineering calculations
    pub fn properties(&self) -> MaterialProperties {
        match self {
            Material::Steel => MaterialProperties {
                density: 7850.0,
                tensile_strength: 400.0,
                yield_strength: 250.0,
                modulus_of_elasticity: 200.0,
                fatigue_limit: 200.0,
                thermal_expansion: 12.0,
                corrosion_resistance: CorrosionResistance::RequiresProtection,
                workability: Workability {
                    machinability: 7.0,
                    weldability: 9.0,
                    formability: 8.0,
                },
                cost_per_kg: 2.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.95,
                    carbon_footprint: 2.3,
                    energy_to_produce: 25.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::StainlessSteel => MaterialProperties {
                density: 8000.0,
                tensile_strength: 515.0,
                yield_strength: 205.0,
                modulus_of_elasticity: 200.0,
                fatigue_limit: 240.0,
                thermal_expansion: 17.3,
                corrosion_resistance: CorrosionResistance::Excellent,
                workability: Workability {
                    machinability: 5.0,
                    weldability: 6.0,
                    formability: 6.0,
                },
                cost_per_kg: 8.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.90,
                    carbon_footprint: 6.15,
                    energy_to_produce: 56.7,
                    biodegradability: 0.0,
                },
            },
            
            Material::Aluminum6061 => MaterialProperties {
                density: 2700.0,
                tensile_strength: 310.0,
                yield_strength: 276.0,
                modulus_of_elasticity: 69.0,
                fatigue_limit: 96.5,
                thermal_expansion: 23.1,
                corrosion_resistance: CorrosionResistance::Good,
                workability: Workability {
                    machinability: 8.0,
                    weldability: 7.0,
                    formability: 9.0,
                },
                cost_per_kg: 4.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.95,
                    carbon_footprint: 11.5,
                    energy_to_produce: 155.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::Aluminum7075 => MaterialProperties {
                density: 2810.0,
                tensile_strength: 572.0,
                yield_strength: 503.0,
                modulus_of_elasticity: 72.0,
                fatigue_limit: 159.0,
                thermal_expansion: 23.2,
                corrosion_resistance: CorrosionResistance::Fair,
                workability: Workability {
                    machinability: 6.0,
                    weldability: 3.0,
                    formability: 5.0,
                },
                cost_per_kg: 6.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.95,
                    carbon_footprint: 11.5,
                    energy_to_produce: 155.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::Titanium => MaterialProperties {
                density: 4500.0,
                tensile_strength: 950.0,
                yield_strength: 880.0,
                modulus_of_elasticity: 114.0,
                fatigue_limit: 510.0,
                thermal_expansion: 8.6,
                corrosion_resistance: CorrosionResistance::Excellent,
                workability: Workability {
                    machinability: 3.0,
                    weldability: 4.0,
                    formability: 4.0,
                },
                cost_per_kg: 35.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.99,
                    carbon_footprint: 35.0,
                    energy_to_produce: 300.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::CarbonFiber => MaterialProperties {
                density: 1600.0,
                tensile_strength: 3500.0,
                yield_strength: 3500.0, // No yield point for carbon fiber
                modulus_of_elasticity: 230.0,
                fatigue_limit: 2800.0,
                thermal_expansion: -0.5, // Negative expansion in fiber direction
                corrosion_resistance: CorrosionResistance::Excellent,
                workability: Workability {
                    machinability: 2.0,
                    weldability: 1.0, // Not weldable
                    formability: 10.0, // Excellent when molded
                },
                cost_per_kg: 50.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.20,
                    carbon_footprint: 31.0,
                    energy_to_produce: 286.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::CarbonFiberHighModulus => MaterialProperties {
                density: 1650.0,
                tensile_strength: 2500.0,
                yield_strength: 2500.0,
                modulus_of_elasticity: 400.0,
                fatigue_limit: 2000.0,
                thermal_expansion: -1.2,
                corrosion_resistance: CorrosionResistance::Excellent,
                workability: Workability {
                    machinability: 1.0,
                    weldability: 1.0,
                    formability: 8.0,
                },
                cost_per_kg: 120.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.15,
                    carbon_footprint: 45.0,
                    energy_to_produce: 400.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::Magnesium => MaterialProperties {
                density: 1740.0,
                tensile_strength: 290.0,
                yield_strength: 220.0,
                modulus_of_elasticity: 45.0,
                fatigue_limit: 100.0,
                thermal_expansion: 26.0,
                corrosion_resistance: CorrosionResistance::Poor,
                workability: Workability {
                    machinability: 9.0,
                    weldability: 5.0,
                    formability: 7.0,
                },
                cost_per_kg: 15.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 0.95,
                    carbon_footprint: 24.7,
                    energy_to_produce: 350.0,
                    biodegradability: 0.0,
                },
            },
            
            Material::Bamboo => MaterialProperties {
                density: 600.0,
                tensile_strength: 370.0,
                yield_strength: 300.0,
                modulus_of_elasticity: 20.0,
                fatigue_limit: 150.0,
                thermal_expansion: 5.0,
                corrosion_resistance: CorrosionResistance::Fair,
                workability: Workability {
                    machinability: 8.0,
                    weldability: 1.0, // Not applicable
                    formability: 9.0,
                },
                cost_per_kg: 3.0,
                environmental_impact: EnvironmentalImpact {
                    recyclability: 1.0,
                    carbon_footprint: -1.0, // Carbon negative
                    energy_to_produce: 2.0,
                    biodegradability: 1.0,
                },
            },
            
            _ => {
                // Default properties for unlisted materials
                MaterialProperties {
                    density: 2700.0,
                    tensile_strength: 300.0,
                    yield_strength: 250.0,
                    modulus_of_elasticity: 70.0,
                    fatigue_limit: 100.0,
                    thermal_expansion: 20.0,
                    corrosion_resistance: CorrosionResistance::Fair,
                    workability: Workability {
                        machinability: 5.0,
                        weldability: 5.0,
                        formability: 5.0,
                    },
                    cost_per_kg: 10.0,
                    environmental_impact: EnvironmentalImpact {
                        recyclability: 0.5,
                        carbon_footprint: 20.0,
                        energy_to_produce: 100.0,
                        biodegradability: 0.1,
                    },
                }
            }
        }
    }
    
    /// Calculate strength-to-weight ratio
    pub fn strength_to_weight_ratio(&self) -> f64 {
        let props = self.properties();
        props.tensile_strength / (props.density / 1000.0) // MPa per g/cm³
    }
    
    /// Calculate stiffness-to-weight ratio
    pub fn stiffness_to_weight_ratio(&self) -> f64 {
        let props = self.properties();
        props.modulus_of_elasticity / (props.density / 1000.0) // GPa per g/cm³
    }
    
    /// Get material suitability for different applications
    pub fn suitability_score(&self, application: MaterialApplication) -> f64 {
        let props = self.properties();
        
        match application {
            MaterialApplication::Frame => {
                let strength_weight = self.strength_to_weight_ratio() / 200.0;
                let stiffness_weight = self.stiffness_to_weight_ratio() / 100.0;
                let fatigue = props.fatigue_limit / 300.0;
                let cost = 1.0 / (props.cost_per_kg / 10.0);
                
                (strength_weight + stiffness_weight + fatigue + cost) / 4.0 * 10.0
            },
            
            MaterialApplication::Wheels => {
                let strength_weight = self.strength_to_weight_ratio() / 150.0;
                let stiffness = props.modulus_of_elasticity / 200.0;
                let fatigue = props.fatigue_limit / 200.0;
                
                (strength_weight + stiffness + fatigue) / 3.0 * 10.0
            },
            
            MaterialApplication::Drivetrain => {
                let strength = props.tensile_strength / 500.0;
                let corrosion = match props.corrosion_resistance {
                    CorrosionResistance::Excellent => 1.0,
                    CorrosionResistance::Good => 0.8,
                    CorrosionResistance::Fair => 0.6,
                    CorrosionResistance::Poor => 0.4,
                    CorrosionResistance::RequiresProtection => 0.5,
                };
                let machinability = props.workability.machinability / 10.0;
                
                (strength + corrosion + machinability) / 3.0 * 10.0
            },
            
            MaterialApplication::Braking => {
                let thermal_stability = 1.0 / (props.thermal_expansion / 20.0);
                let strength = props.tensile_strength / 400.0;
                let corrosion = match props.corrosion_resistance {
                    CorrosionResistance::Excellent => 1.0,
                    CorrosionResistance::Good => 0.8,
                    _ => 0.5,
                };
                
                (thermal_stability + strength + corrosion) / 3.0 * 10.0
            },
        }
    }
    
    /// Calculate environmental score (higher is better)
    pub fn environmental_score(&self) -> f64 {
        let impact = self.properties().environmental_impact;
        let recyclability_score = impact.recyclability * 30.0;
        let carbon_score = (1.0 / (impact.carbon_footprint + 1.0)) * 40.0;
        let energy_score = (1.0 / (impact.energy_to_produce / 100.0 + 1.0)) * 20.0;
        let biodegradability_score = impact.biodegradability * 10.0;
        
        recyclability_score + carbon_score + energy_score + biodegradability_score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialApplication {
    Frame,
    Wheels,
    Drivetrain,
    Braking,
}

/// Material comparison for selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialComparison {
    pub materials: Vec<(Material, f64)>, // Material and score
    pub criteria: MaterialSelectionCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialSelectionCriteria {
    pub weight_importance: f64,     // 0-1
    pub strength_importance: f64,   // 0-1
    pub cost_importance: f64,       // 0-1
    pub environmental_importance: f64, // 0-1
    pub durability_importance: f64, // 0-1
}

impl MaterialComparison {
    pub fn new(criteria: MaterialSelectionCriteria) -> Self {
        Self {
            materials: Vec::new(),
            criteria,
        }
    }
    
    pub fn evaluate_material(&mut self, material: Material, application: MaterialApplication) {
        let props = material.properties();
        let suitability = material.suitability_score(application);
        let environmental = material.environmental_score();
        
        // Weighted scoring
        let weight_score = (1.0 / (props.density / 1000.0)) * self.criteria.weight_importance;
        let strength_score = (props.tensile_strength / 1000.0) * self.criteria.strength_importance;
        let cost_score = (1.0 / props.cost_per_kg) * self.criteria.cost_importance;
        let env_score = (environmental / 100.0) * self.criteria.environmental_importance;
        let durability_score = (props.fatigue_limit / 500.0) * self.criteria.durability_importance;
        
        let total_score = weight_score + strength_score + cost_score + env_score + durability_score;
        
        self.materials.push((material, total_score));
    }
    
    pub fn get_best_materials(&mut self, count: usize) -> Vec<(Material, f64)> {
        self.materials.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.materials.iter().take(count).cloned().collect()
    }
}

/// Material database for quick lookups
pub struct MaterialDatabase {
    materials: Vec<Material>,
}

impl MaterialDatabase {
    pub fn new() -> Self {
        Self {
            materials: vec![
                Material::Steel,
                Material::StainlessSteel,
                Material::Aluminum6061,
                Material::Aluminum7075,
                Material::Titanium,
                Material::CarbonFiber,
                Material::CarbonFiberHighModulus,
                Material::Magnesium,
                Material::Bamboo,
            ],
        }
    }
    
    pub fn get_materials_for_application(&self, application: MaterialApplication) -> Vec<Material> {
        self.materials.iter()
            .filter(|material| material.suitability_score(application.clone()) > 5.0)
            .cloned()
            .collect()
    }
    
    pub fn get_eco_friendly_materials(&self) -> Vec<Material> {
        self.materials.iter()
            .filter(|material| material.environmental_score() > 60.0)
            .cloned()
            .collect()
    }
    
    pub fn get_budget_materials(&self, max_cost_per_kg: f64) -> Vec<Material> {
        self.materials.iter()
            .filter(|material| material.properties().cost_per_kg <= max_cost_per_kg)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_properties() {
        let aluminum = Material::Aluminum6061;
        let props = aluminum.properties();
        assert!(props.density > 0.0);
        assert!(props.tensile_strength > 0.0);
    }

    #[test]
    fn test_strength_to_weight_ratio() {
        let carbon = Material::CarbonFiber;
        let aluminum = Material::Aluminum6061;
        
        assert!(carbon.strength_to_weight_ratio() > aluminum.strength_to_weight_ratio());
    }

    #[test]
    fn test_material_comparison() {
        let criteria = MaterialSelectionCriteria {
            weight_importance: 0.3,
            strength_importance: 0.3,
            cost_importance: 0.2,
            environmental_importance: 0.1,
            durability_importance: 0.1,
        };
        
        let mut comparison = MaterialComparison::new(criteria);
        comparison.evaluate_material(Material::Aluminum6061, MaterialApplication::Frame);
        comparison.evaluate_material(Material::CarbonFiber, MaterialApplication::Frame);
        
        let best = comparison.get_best_materials(1);
        assert!(!best.is_empty());
    }
}
