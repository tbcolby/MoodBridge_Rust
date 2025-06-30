//! # Bicycle Physics Engine
//!
//! This module implements realistic physics calculations for bicycle dynamics,
//! including forces, motion, stability, and performance characteristics.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Gravitational constants and calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gravity {
    pub acceleration: f64, // m/s² (Earth = 9.81)
    pub slope_angle: f64,  // degrees
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            acceleration: 9.81,
            slope_angle: 0.0,
        }
    }
}

impl Gravity {
    pub fn force_component(&self, mass: f64) -> f64 {
        mass * self.acceleration * (self.slope_angle.to_radians().sin())
    }
}

/// Friction and rolling resistance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friction {
    pub coefficient: f64,     // Rolling resistance coefficient
    pub surface_type: SurfaceType,
    pub tire_pressure: f64,   // PSI
    pub temperature: f64,     // Celsius
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    Asphalt,
    Concrete,
    Gravel,
    Dirt,
    Sand,
    Wet,
    Ice,
}

impl Default for Friction {
    fn default() -> Self {
        Self {
            coefficient: 0.005, // Typical road bike on asphalt
            surface_type: SurfaceType::Asphalt,
            tire_pressure: 100.0,
            temperature: 20.0,
        }
    }
}

impl Friction {
    pub fn calculate_resistance(&self, normal_force: f64, speed: f64) -> f64 {
        let base_resistance = self.coefficient * normal_force;
        let speed_factor = 1.0 + (speed / 50.0).powi(2) * 0.1; // Speed-dependent increase
        let pressure_factor = self.tire_pressure / 100.0; // Lower pressure = higher resistance
        
        base_resistance * speed_factor / pressure_factor
    }
    
    pub fn adjust_for_surface(&mut self) {
        self.coefficient = match self.surface_type {
            SurfaceType::Asphalt => 0.004,
            SurfaceType::Concrete => 0.005,
            SurfaceType::Gravel => 0.015,
            SurfaceType::Dirt => 0.025,
            SurfaceType::Sand => 0.050,
            SurfaceType::Wet => 0.008,
            SurfaceType::Ice => 0.020,
        };
    }
}

/// Wind resistance and aerodynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindResistance {
    pub coefficient: f64,         // Air resistance coefficient
    pub frontal_area: f64,        // m²
    pub air_density: f64,         // kg/m³
    pub wind_speed: f64,          // m/s (headwind positive)
    pub altitude: f64,            // meters above sea level
}

impl Default for WindResistance {
    fn default() -> Self {
        Self {
            coefficient: 0.9,        // Typical road cycling position
            frontal_area: 0.4,       // m² for road cyclist
            air_density: 1.225,      // kg/m³ at sea level, 15°C
            wind_speed: 0.0,
            altitude: 0.0,
        }
    }
}

impl WindResistance {
    pub fn calculate_drag_force(&self, velocity: f64) -> f64 {
        let relative_velocity = velocity + self.wind_speed;
        let adjusted_density = self.air_density * (1.0 - self.altitude / 10000.0);
        
        0.5 * self.coefficient * self.frontal_area * adjusted_density * relative_velocity.powi(2)
    }
    
    pub fn adjust_for_position(&mut self, position: RidingPosition) {
        let (coeff, area) = match position {
            RidingPosition::Upright => (1.2, 0.6),
            RidingPosition::Relaxed => (1.0, 0.5),
            RidingPosition::Aggressive => (0.8, 0.35),
            RidingPosition::TimeTrialAero => (0.6, 0.25),
        };
        self.coefficient = coeff;
        self.frontal_area = area;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RidingPosition {
    Upright,
    Relaxed,
    Aggressive,
    TimeTrialAero,
}

/// Bicycle dynamics and motion calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleDynamics {
    pub mass: f64,              // kg (bike + rider)
    pub wheelbase: f64,         // m
    pub trail: f64,             // m
    pub center_of_gravity: CenterOfGravity,
    pub moments_of_inertia: MomentsOfInertia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterOfGravity {
    pub height: f64,            // m above ground
    pub longitudinal: f64,      // m from rear axle
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentsOfInertia {
    pub pitch: f64,             // kg⋅m² (front/back rotation)
    pub roll: f64,              // kg⋅m² (side-to-side rotation)
    pub yaw: f64,               // kg⋅m² (steering rotation)
}

impl BicycleDynamics {
    pub fn calculate_stability(&self, speed: f64) -> StabilityMetrics {
        let weave_speed = self.calculate_weave_speed();
        let capsize_speed = self.calculate_capsize_speed();
        
        StabilityMetrics {
            weave_speed,
            capsize_speed,
            stable_speed_range: (weave_speed, capsize_speed),
            current_stability: self.evaluate_stability_at_speed(speed),
        }
    }
    
    fn calculate_weave_speed(&self) -> f64 {
        // Simplified weave mode calculation
        // Real calculation involves complex eigenvalue analysis
        4.0 * (self.wheelbase / self.trail).sqrt()
    }
    
    fn calculate_capsize_speed(&self) -> f64 {
        // Simplified capsize mode calculation
        15.0 * (self.trail / 0.06).sqrt()
    }
    
    fn evaluate_stability_at_speed(&self, speed: f64) -> f64 {
        let weave = self.calculate_weave_speed();
        let capsize = self.calculate_capsize_speed();
        
        if speed < weave {
            // Below weave speed - less stable
            speed / weave * 0.7
        } else if speed > capsize {
            // Above capsize speed - less stable
            1.0 - ((speed - capsize) / capsize * 0.5)
        } else {
            // In stable range
            1.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityMetrics {
    pub weave_speed: f64,       // km/h - speed below which weave mode is unstable
    pub capsize_speed: f64,     // km/h - speed above which capsize mode is unstable
    pub stable_speed_range: (f64, f64), // km/h range of stable riding
    pub current_stability: f64, // 0.0-1.0 stability rating at current speed
}

/// Power and performance calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerAnalysis {
    pub rider_power: f64,       // Watts
    pub mechanical_efficiency: f64, // 0.0-1.0
    pub drivetrain_efficiency: f64, // 0.0-1.0
    pub metabolic_efficiency: f64,  // 0.0-1.0
}

impl PowerAnalysis {
    pub fn calculate_speed(&self, total_resistance: f64) -> f64 {
        let available_power = self.rider_power * self.mechanical_efficiency * self.drivetrain_efficiency;
        
        if total_resistance > 0.0 {
            available_power / total_resistance
        } else {
            0.0
        }
    }
    
    pub fn calculate_climbing_speed(&self, gradient: f64, mass: f64, gravity: &Gravity) -> f64 {
        let climbing_force = mass * gravity.acceleration * (gradient.to_radians().sin());
        let available_power = self.rider_power * self.mechanical_efficiency * self.drivetrain_efficiency;
        
        if climbing_force > 0.0 {
            available_power / climbing_force * 3.6 // Convert m/s to km/h
        } else {
            0.0
        }
    }
}

/// Comprehensive physics simulation
pub struct PhysicsEngine {
    pub gravity: Gravity,
    pub friction: Friction,
    pub wind_resistance: WindResistance,
    pub dynamics: BicycleDynamics,
    pub power_analysis: PowerAnalysis,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            gravity: Gravity::default(),
            friction: Friction::default(),
            wind_resistance: WindResistance::default(),
            dynamics: BicycleDynamics::default(),
            power_analysis: PowerAnalysis::default(),
        }
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsEngine {
    pub fn simulate_motion(&self, time_step: f64, duration: f64) -> MotionSimulation {
        let mut simulation = MotionSimulation::new();
        let mut current_speed = 0.0;
        let mut current_time = 0.0;
        
        while current_time < duration {
            // Calculate forces
            let weight_force = self.dynamics.mass * self.gravity.acceleration;
            let rolling_resistance = self.friction.calculate_resistance(weight_force, current_speed);
            let air_resistance = self.wind_resistance.calculate_drag_force(current_speed);
            let total_resistance = rolling_resistance + air_resistance;
            
            // Calculate acceleration
            let net_force = self.power_analysis.rider_power / current_speed.max(1.0) - total_resistance;
            let acceleration = net_force / self.dynamics.mass;
            
            // Update motion
            current_speed += acceleration * time_step;
            current_speed = current_speed.max(0.0); // Can't go backwards
            
            // Record data point
            simulation.add_data_point(current_time, current_speed, acceleration);
            
            current_time += time_step;
        }
        
        simulation
    }
}

impl Default for BicycleDynamics {
    fn default() -> Self {
        Self {
            mass: 80.0, // 75kg rider + 5kg bike
            wheelbase: 1.0,
            trail: 0.06,
            center_of_gravity: CenterOfGravity {
                height: 1.0,
                longitudinal: 0.4,
            },
            moments_of_inertia: MomentsOfInertia {
                pitch: 3.0,
                roll: 0.5,
                yaw: 0.3,
            },
        }
    }
}

impl Default for PowerAnalysis {
    fn default() -> Self {
        Self {
            rider_power: 250.0, // Watts - recreational cyclist
            mechanical_efficiency: 0.24, // Human muscle efficiency
            drivetrain_efficiency: 0.98, // Modern drivetrain
            metabolic_efficiency: 0.22, // Overall human efficiency
        }
    }
}

/// Motion simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionSimulation {
    pub time_points: Vec<f64>,
    pub speed_points: Vec<f64>,
    pub acceleration_points: Vec<f64>,
    pub max_speed: f64,
    pub time_to_max_speed: f64,
}

impl MotionSimulation {
    pub fn new() -> Self {
        Self {
            time_points: Vec::new(),
            speed_points: Vec::new(),
            acceleration_points: Vec::new(),
            max_speed: 0.0,
            time_to_max_speed: 0.0,
        }
    }
    
    pub fn add_data_point(&mut self, time: f64, speed: f64, acceleration: f64) {
        self.time_points.push(time);
        self.speed_points.push(speed);
        self.acceleration_points.push(acceleration);
        
        if speed > self.max_speed {
            self.max_speed = speed;
            self.time_to_max_speed = time;
        }
    }
}

/// Braking physics calculations
pub fn calculate_braking_distance(initial_speed: f64, brake_force: f64, mass: f64, friction_coeff: f64) -> f64 {
    // Using kinematic equations: v² = u² + 2as
    // Where final velocity = 0, u = initial_speed, a = deceleration
    let max_friction_force = mass * 9.81 * friction_coeff;
    let total_brake_force = (brake_force + max_friction_force).min(max_friction_force * 1.2);
    let deceleration = total_brake_force / mass;
    
    if deceleration > 0.0 {
        (initial_speed.powi(2)) / (2.0 * deceleration)
    } else {
        f64::INFINITY
    }
}

/// Gear ratio and mechanical advantage calculations
pub fn calculate_gear_ratio(chainring_teeth: u32, cassette_teeth: u32) -> f64 {
    chainring_teeth as f64 / cassette_teeth as f64
}

pub fn calculate_wheel_speed(crank_rpm: f64, gear_ratio: f64, wheel_circumference: f64) -> f64 {
    // Speed in m/s
    (crank_rpm * gear_ratio * wheel_circumference) / 60.0
}

pub fn calculate_mechanical_advantage(gear_ratio: f64, wheel_radius: f64, crank_length: f64) -> f64 {
    gear_ratio * (wheel_radius / crank_length)
}

/// Suspension physics (for mountain bikes)
pub fn calculate_suspension_response(spring_rate: f64, damping_coefficient: f64, input_force: f64) -> f64 {
    // Simplified spring-damper system response
    let natural_frequency = (spring_rate / 1.0).sqrt(); // Assuming 1kg sprung mass
    let damping_ratio = damping_coefficient / (2.0 * natural_frequency);
    
    // Response amplitude (simplified)
    input_force / spring_rate / (1.0 + damping_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_force() {
        let gravity = Gravity {
            acceleration: 9.81,
            slope_angle: 10.0,
        };
        let force = gravity.force_component(75.0);
        assert!((force - 127.9).abs() < 1.0); // Approximate value
    }

    #[test]
    fn test_wind_resistance() {
        let wind = WindResistance::default();
        let drag = wind.calculate_drag_force(15.0); // 15 m/s ≈ 54 km/h
        assert!(drag > 0.0);
        assert!(drag < 1000.0); // Reasonable bounds
    }

    #[test]
    fn test_gear_ratio() {
        let ratio = calculate_gear_ratio(50, 25);
        assert_eq!(ratio, 2.0);
    }

    #[test]
    fn test_braking_distance() {
        let distance = calculate_braking_distance(15.0, 500.0, 75.0, 0.7);
        assert!(distance > 0.0);
        assert!(distance < 100.0); // Should be reasonable stopping distance
    }
}
