//! # Aerodynamics Module
//!
//! This module provides aerodynamic analysis for bicycle designs,
//! including drag calculations, wind tunnel simulation, and position optimization.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use super::design::BicycleDesign;

/// Aerodynamic analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AerodynamicAnalysis {
    pub drag_coefficient: f64,
    pub frontal_area: f64,        // m²
    pub drag_force_at_40kmh: f64, // N
    pub power_required_at_40kmh: f64, // W
    pub pressure_distribution: Vec<PressurePoint>,
    pub flow_visualization: FlowVisualization,
    pub recommendations: Vec<String>,
}

/// Pressure point for CFD visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressurePoint {
    pub x: f64,           // Position
    pub y: f64,
    pub pressure: f64,    // Pa
    pub velocity: f64,    // m/s
}

/// Flow visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowVisualization {
    pub streamlines: Vec<Streamline>,
    pub vortex_regions: Vec<VortexRegion>,
    pub separation_points: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Streamline {
    pub points: Vec<(f64, f64)>,
    pub velocity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VortexRegion {
    pub center: (f64, f64),
    pub radius: f64,
    pub strength: f64,
}

/// Wind tunnel test conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindTunnelConditions {
    pub wind_speed: f64,      // m/s
    pub air_density: f64,     // kg/m³
    pub temperature: f64,     // K
    pub pressure: f64,        // Pa
    pub yaw_angle: f64,       // degrees (crosswind)
    pub turbulence_intensity: f64, // %
}

impl Default for WindTunnelConditions {
    fn default() -> Self {
        Self {
            wind_speed: 11.11, // 40 km/h
            air_density: 1.225,
            temperature: 288.15, // 15°C
            pressure: 101325.0,
            yaw_angle: 0.0,
            turbulence_intensity: 0.5,
        }
    }
}

/// Aerodynamic analyzer
pub struct AerodynamicAnalyzer {
    pub conditions: WindTunnelConditions,
}

impl Default for AerodynamicAnalyzer {
    fn default() -> Self {
        Self {
            conditions: WindTunnelConditions::default(),
        }
    }
}

impl AerodynamicAnalyzer {
    pub fn new(conditions: WindTunnelConditions) -> Self {
        Self { conditions }
    }

    /// Perform comprehensive aerodynamic analysis
    pub fn analyze(&self, design: &BicycleDesign) -> AerodynamicAnalysis {
        let drag_coefficient = self.calculate_drag_coefficient(design);
        let frontal_area = self.calculate_frontal_area(design);
        let drag_force = self.calculate_drag_force(drag_coefficient, frontal_area);
        let power_required = drag_force * self.conditions.wind_speed;
        
        let pressure_distribution = self.simulate_pressure_distribution(design);
        let flow_visualization = self.generate_flow_visualization(design);
        let recommendations = self.generate_recommendations(design, drag_coefficient);

        AerodynamicAnalysis {
            drag_coefficient,
            frontal_area,
            drag_force_at_40kmh: drag_force,
            power_required_at_40kmh: power_required,
            pressure_distribution,
            flow_visualization,
            recommendations,
        }
    }

    fn calculate_drag_coefficient(&self, design: &BicycleDesign) -> f64 {
        let mut cd = 0.3; // Base drag coefficient for a bicycle
        
        // Frame contribution
        cd += match design.frame.tube_set.down_tube.shape {
            super::design::TubeShape::Round => 0.05,
            super::design::TubeShape::Aero => -0.02,
            super::design::TubeShape::Oval => 0.01,
            _ => 0.03,
        };
        
        // Wheel contribution
        cd += match design.wheels.front_wheel.rim.aerodynamic_profile {
            super::design::AeroProfile::Traditional => 0.08,
            super::design::AeroProfile::SemiAero => 0.06,
            super::design::AeroProfile::DeepSection => 0.04,
            super::design::AeroProfile::Disc => 0.02,
        };
        
        // Handlebar contribution
        cd += match design.steering.handlebars.style {
            super::design::HandlebarStyle::Drop => 0.02,
            super::design::HandlebarStyle::Aero => -0.01,
            super::design::HandlebarStyle::Flat => 0.04,
            _ => 0.03,
        };
        
        // Yaw angle effect
        let yaw_factor = 1.0 + (self.conditions.yaw_angle.to_radians().sin().abs() * 0.2);
        cd *= yaw_factor;
        
        cd.max(0.1) // Minimum realistic Cd
    }

    fn calculate_frontal_area(&self, design: &BicycleDesign) -> f64 {
        // Base frontal area for bicycle + rider
        let mut area = 0.4; // m² typical for road cyclist
        
        // Adjust based on bicycle type
        area *= match design.intended_use {
            super::design::BicycleType::Road => 1.0,
            super::design::BicycleType::Mountain => 1.15,
            super::design::BicycleType::Touring => 1.1,
            super::design::BicycleType::Commuter => 1.2,
            super::design::BicycleType::BMX => 0.9,
            _ => 1.05,
        };
        
        // Handlebar width effect
        let bar_width_factor = design.steering.handlebars.width / 420.0; // Relative to standard 420mm
        area *= 0.95 + (bar_width_factor * 0.1);
        
        area
    }

    fn calculate_drag_force(&self, cd: f64, area: f64) -> f64 {
        0.5 * self.conditions.air_density * self.conditions.wind_speed.powi(2) * cd * area
    }

    fn simulate_pressure_distribution(&self, design: &BicycleDesign) -> Vec<PressurePoint> {
        let mut points = Vec::new();
        let num_points = 50;
        
        // Simulate pressure around the bicycle profile
        for i in 0..num_points {
            let angle = (i as f64 / num_points as f64) * 2.0 * PI;
            let x = angle.cos() * 0.5; // Simplified bicycle profile
            let y = angle.sin() * 0.8;
            
            // Simplified pressure calculation based on potential flow
            let velocity_factor = 1.0 + 0.5 * (2.0 * angle).sin();
            let pressure = self.conditions.pressure * (1.0 - velocity_factor.powi(2) * 0.1);
            let velocity = self.conditions.wind_speed * velocity_factor;
            
            points.push(PressurePoint {
                x,
                y,
                pressure,
                velocity,
            });
        }
        
        points
    }

    fn generate_flow_visualization(&self, design: &BicycleDesign) -> FlowVisualization {
        let mut streamlines = Vec::new();
        let mut vortex_regions = Vec::new();
        let separation_points = vec![(0.3, 0.2), (0.3, -0.2)]; // Simplified separation points
        
        // Generate streamlines
        for i in 0..10 {
            let y_start = -0.5 + (i as f64 / 9.0);
            let mut points = Vec::new();
            
            for j in 0..20 {
                let x = -1.0 + (j as f64 / 19.0) * 2.0;
                let y = y_start + 0.1 * (PI * x).sin(); // Simplified streamline
                points.push((x, y));
            }
            
            streamlines.push(Streamline {
                points,
                velocity: self.conditions.wind_speed,
            });
        }
        
        // Add vortex behind bicycle
        vortex_regions.push(VortexRegion {
            center: (0.5, 0.0),
            radius: 0.1,
            strength: -2.0,
        });
        
        FlowVisualization {
            streamlines,
            vortex_regions,
            separation_points,
        }
    }

    fn generate_recommendations(&self, design: &BicycleDesign, cd: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if cd > 0.4 {
            recommendations.push("High drag coefficient detected. Consider aerodynamic improvements.".to_string());
        }
        
        match design.wheels.front_wheel.rim.aerodynamic_profile {
            super::design::AeroProfile::Traditional => {
                recommendations.push("Upgrade to aerodynamic wheels for 3-5% drag reduction".to_string());
            },
            _ => {},
        }
        
        match design.frame.tube_set.down_tube.shape {
            super::design::TubeShape::Round => {
                recommendations.push("Consider aero frame tubing for improved aerodynamics".to_string());
            },
            _ => {},
        }
        
        match design.steering.handlebars.style {
            super::design::HandlebarStyle::Flat | super::design::HandlebarStyle::Riser => {
                recommendations.push("Drop handlebars can reduce aerodynamic drag".to_string());
            },
            _ => {},
        }
        
        if design.intended_use == super::design::BicycleType::Road && cd > 0.35 {
            recommendations.push("For road cycling, aim for Cd < 0.35 for competitive performance".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("Aerodynamic performance is good for this bicycle type".to_string());
        }
        
        recommendations
    }

    /// Compare aerodynamic performance at different speeds
    pub fn speed_analysis(&self, design: &BicycleDesign, speeds: Vec<f64>) -> Vec<(f64, f64, f64)> {
        let cd = self.calculate_drag_coefficient(design);
        let area = self.calculate_frontal_area(design);
        
        speeds.into_iter().map(|speed| {
            let drag_force = 0.5 * self.conditions.air_density * speed.powi(2) * cd * area;
            let power_required = drag_force * speed;
            (speed * 3.6, drag_force, power_required) // Convert m/s to km/h
        }).collect()
    }

    /// Calculate power savings from aerodynamic improvements
    pub fn calculate_aero_savings(&self, original: &BicycleDesign, improved: &BicycleDesign) -> f64 {
        let original_analysis = self.analyze(original);
        let improved_analysis = self.analyze(improved);
        
        let power_savings = original_analysis.power_required_at_40kmh - improved_analysis.power_required_at_40kmh;
        (power_savings / original_analysis.power_required_at_40kmh) * 100.0 // Percentage savings
    }
}

/// Yaw angle testing for crosswind analysis
pub fn yaw_sweep_analysis(design: &BicycleDesign, yaw_angles: Vec<f64>) -> Vec<(f64, f64)> {
    yaw_angles.into_iter().map(|yaw| {
        let conditions = WindTunnelConditions {
            yaw_angle: yaw,
            ..Default::default()
        };
        let analyzer = AerodynamicAnalyzer::new(conditions);
        let analysis = analyzer.analyze(design);
        (yaw, analysis.drag_coefficient)
    }).collect()
}

/// Calculate Reynolds number for flow analysis
pub fn calculate_reynolds_number(velocity: f64, characteristic_length: f64, kinematic_viscosity: f64) -> f64 {
    velocity * characteristic_length / kinematic_viscosity
}

/// Estimate power requirements for different riding positions
pub fn position_comparison(design: &BicycleDesign) -> Vec<(String, f64)> {
    let analyzer = AerodynamicAnalyzer::default();
    
    vec![
        ("Upright".to_string(), 0.5),
        ("Relaxed".to_string(), 0.4),
        ("Aggressive".to_string(), 0.35),
        ("Time Trial".to_string(), 0.25),
    ].into_iter().map(|(position, area_factor)| {
        let mut analysis = analyzer.analyze(design);
        analysis.frontal_area *= area_factor;
        let power = 0.5 * 1.225 * (11.11_f64).powi(2) * analysis.drag_coefficient * analysis.frontal_area * 11.11;
        (position, power)
    }).collect()
}
