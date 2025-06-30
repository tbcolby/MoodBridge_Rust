//! # Bicycle Design Module
//!
//! This module defines the core bicycle design structures, including frame geometry,
//! components, materials, and design validation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::bicycle::materials::Material;

/// Complete bicycle design specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicycleDesign {
    pub id: Uuid,
    pub name: String,
    pub version: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    
    // Core Design Components
    pub frame: Frame,
    pub wheels: WheelSet,
    pub drivetrain: Drivetrain,
    pub braking: BrakingSystem,
    pub steering: SteeringSystem,
    pub suspension: Option<SuspensionSystem>,
    
    // Design Properties
    pub intended_use: BicycleType,
    pub target_weight: f64,  // kg
    pub target_price: f64,   // USD
    pub rider_fit: RiderFit,
    
    // Validation Status
    pub validation_status: ValidationStatus,
    pub design_notes: Vec<String>,
}

/// Frame geometry and construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub geometry: FrameGeometry,
    pub material: Material,
    pub construction_method: ConstructionMethod,
    pub tube_set: TubeSet,
    pub weight: f64, // kg
    pub stiffness_rating: f64, // N⋅m²
    pub fatigue_life: u32, // cycles
}

/// Critical frame geometry measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameGeometry {
    // Primary measurements (mm)
    pub seat_tube_length: f64,
    pub top_tube_length: f64,
    pub head_tube_length: f64,
    pub chainstay_length: f64,
    pub wheelbase: f64,
    
    // Angles (degrees)
    pub head_tube_angle: f64,
    pub seat_tube_angle: f64,
    
    // Height measurements (mm)
    pub bottom_bracket_height: f64,
    pub stack: f64,
    pub reach: f64,
    
    // Derived measurements
    pub trail: f64,
    pub mechanical_trail: f64,
    pub front_center: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstructionMethod {
    Welded,
    Brazed,
    Bonded,
    Molded,
    Additive, // 3D printed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TubeSet {
    pub seat_tube: TubeSpec,
    pub top_tube: TubeSpec,
    pub down_tube: TubeSpec,
    pub head_tube: TubeSpec,
    pub chainstays: TubeSpec,
    pub seatstays: TubeSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TubeSpec {
    pub diameter: f64,    // mm
    pub wall_thickness: f64, // mm
    pub shape: TubeShape,
    pub material: Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TubeShape {
    Round,
    Oval,
    Aero,
    Square,
    Custom(String),
}

/// Wheel specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelSet {
    pub front_wheel: Wheel,
    pub rear_wheel: Wheel,
    pub tire_clearance: f64, // mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wheel {
    pub diameter: f64,      // mm (700c, 650b, 26", 29", etc.)
    pub rim: RimSpec,
    pub hub: HubSpec,
    pub spokes: SpokeSpec,
    pub tire: TireSpec,
    pub weight: f64,        // kg
    pub rotational_inertia: f64, // kg⋅m²
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RimSpec {
    pub material: Material,
    pub depth: f64,         // mm
    pub width: f64,         // mm
    pub spoke_count: u32,
    pub aerodynamic_profile: AeroProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpec {
    pub material: Material,
    pub bearing_type: BearingType,
    pub engagement_points: u32, // for rear hub
    pub weight: f64,            // kg
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokeSpec {
    pub material: Material,
    pub count: u32,
    pub gauge: f64,         // mm
    pub pattern: SpokePattern,
    pub tension: f64,       // N
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TireSpec {
    pub width: f64,         // mm
    pub pressure: f64,      // PSI
    pub compound: TireCompound,
    pub tread_pattern: TreadPattern,
    pub puncture_resistance: u32, // 1-10 scale
    pub rolling_resistance: f64,  // coefficient
}

/// Drivetrain system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drivetrain {
    pub crankset: Crankset,
    pub cassette: Cassette,
    pub chain: Chain,
    pub front_derailleur: Option<Derailleur>,
    pub rear_derailleur: Derailleur,
    pub shifters: ShifterSet,
    pub bottom_bracket: BottomBracket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crankset {
    pub chainrings: Vec<u32>,   // teeth count
    pub crank_length: f64,      // mm
    pub material: Material,
    pub weight: f64,            // kg
    pub power_meter: Option<PowerMeterSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cassette {
    pub speeds: u32,
    pub gear_ratios: Vec<u32>,  // teeth count
    pub material: Material,
    pub weight: f64,            // kg
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub speeds: u32,
    pub material: Material,
    pub length: u32,            // links
    pub efficiency: f64,        // 0.0-1.0
}

/// Braking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrakingSystem {
    pub front_brake: Brake,
    pub rear_brake: Brake,
    pub brake_type: BrakeType,
    pub stopping_power: f64,    // N⋅m
    pub modulation: f64,        // 0.0-1.0 (feel)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brake {
    pub caliper: CaliperSpec,
    pub rotor: Option<RotorSpec>, // for disc brakes
    pub pads: PadSpec,
}

/// Bicycle type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BicycleType {
    Road,
    Mountain,
    Gravel,
    Touring,
    Commuter,
    BMX,
    Track,
    Recumbent,
    Cargo,
    Electric,
    Custom(String),
}

/// Rider fit parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiderFit {
    pub rider_height: f64,      // cm
    pub inseam: f64,           // cm
    pub arm_length: f64,       // cm
    pub torso_length: f64,     // cm
    pub flexibility: FlexibilityLevel,
    pub riding_style: RidingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlexibilityLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RidingStyle {
    Aggressive,
    Balanced,
    Comfort,
    Touring,
}

/// Design validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Draft,
    UnderReview,
    Validated,
    RequiresChanges(Vec<String>),
    Approved,
    InProduction,
}

// Additional enums and structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AeroProfile {
    Traditional,
    SemiAero,
    DeepSection,
    Disc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearingType {
    Loose,
    Cartridge,
    Ceramic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpokePattern {
    Radial,
    Cross1,
    Cross2,
    Cross3,
    Cross4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TireCompound {
    Hard,
    Medium,
    Soft,
    Dual,
    Triple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreadPattern {
    Slick,
    SemiSlick,
    File,
    Knobby,
    Aggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrakeType {
    RimBrake,
    DiscBrake,
    Drum,
    Coaster,
}

// Placeholder types for completeness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derailleur {
    pub speeds: u32,
    pub capacity: u32,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShifterSet {
    pub speeds: u32,
    pub shift_type: ShiftType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShiftType {
    Mechanical,
    Electronic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomBracket {
    pub threading: ThreadingType,
    pub material: Material,
    pub bearing_type: BearingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadingType {
    BSA,
    ITA,
    BB30,
    PF30,
    BBRight,
    T47,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMeterSpec {
    pub accuracy: f64,          // ±%
    pub battery_life: u32,      // hours
    pub measurement_type: PowerMeasurementType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerMeasurementType {
    Strain,
    Hub,
    Pedal,
    Crank,
    Spider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaliperSpec {
    pub piston_count: u32,
    pub material: Material,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotorSpec {
    pub diameter: f64,          // mm
    pub material: Material,
    pub thickness: f64,         // mm
    pub mounting: RotorMounting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotorMounting {
    SixBolt,
    CenterLock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PadSpec {
    pub material: PadMaterial,
    pub life_expectancy: u32,   // km
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PadMaterial {
    Organic,
    Metallic,
    Ceramic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteeringSystem {
    pub headset: HeadsetSpec,
    pub stem: StemSpec,
    pub handlebars: HandlebarSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadsetSpec {
    pub bearing_type: BearingType,
    pub material: Material,
    pub stack_height: f64,      // mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemSpec {
    pub length: f64,            // mm
    pub angle: f64,             // degrees
    pub clamp_diameter: f64,    // mm
    pub material: Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlebarSpec {
    pub style: HandlebarStyle,
    pub width: f64,             // mm
    pub drop: f64,              // mm (for drop bars)
    pub reach: f64,             // mm (for drop bars)
    pub material: Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandlebarStyle {
    Drop,
    Flat,
    Riser,
    Bullhorn,
    Aero,
    BMX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspensionSystem {
    pub front: Option<SuspensionSpec>,
    pub rear: Option<SuspensionSpec>,
    pub total_travel: f64,      // mm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspensionSpec {
    pub travel: f64,            // mm
    pub spring_type: SpringType,
    pub damping: DampingSpec,
    pub lockout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpringType {
    Coil,
    Air,
    Leaf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DampingSpec {
    pub compression: DampingLevel,
    pub rebound: DampingLevel,
    pub adjustable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DampingLevel {
    Soft,
    Medium,
    Firm,
    Custom(f64),
}

impl BicycleDesign {
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            version: 1,
            created_at: now,
            updated_at: now,
            frame: Frame::default(),
            wheels: WheelSet::default(),
            drivetrain: Drivetrain::default(),
            braking: BrakingSystem::default(),
            steering: SteeringSystem::default(),
            suspension: None,
            intended_use: BicycleType::Road,
            target_weight: 8.0, // kg
            target_price: 2000.0, // USD
            rider_fit: RiderFit::default(),
            validation_status: ValidationStatus::Draft,
            design_notes: Vec::new(),
        }
    }

    /// Calculate the bike's total weight
    pub fn calculate_total_weight(&self) -> f64 {
        self.frame.weight +
        self.wheels.front_wheel.weight +
        self.wheels.rear_wheel.weight +
        self.drivetrain.crankset.weight +
        self.drivetrain.cassette.weight +
        0.3 // chain approximate weight
    }

    /// Validate the design for basic consistency
    pub fn validate_design(&self) -> Vec<String> {
        let mut issues = Vec::new();

        // Check basic geometry constraints
        if self.frame.geometry.wheelbase < 900.0 || self.frame.geometry.wheelbase > 1300.0 {
            issues.push("Wheelbase outside normal range (900-1300mm)".to_string());
        }

        if self.frame.geometry.head_tube_angle < 65.0 || self.frame.geometry.head_tube_angle > 75.0 {
            issues.push("Head tube angle outside normal range (65-75°)".to_string());
        }

        // Check weight constraints
        let total_weight = self.calculate_total_weight();
        if total_weight > self.target_weight * 1.2 {
            issues.push(format!("Design weight ({:.1}kg) exceeds target by >20%", total_weight));
        }

        // Check gear ratio compatibility
        if self.drivetrain.cassette.speeds != self.drivetrain.chain.speeds {
            issues.push("Cassette and chain speed mismatch".to_string());
        }

        issues
    }

    /// Get recommended rider size range
    pub fn get_rider_size_range(&self) -> (f64, f64) {
        // Simple approximation based on seat tube length
        let min_height = self.frame.geometry.seat_tube_length * 1.8 + 60.0;
        let max_height = self.frame.geometry.seat_tube_length * 2.1 + 80.0;
        (min_height, max_height)
    }
}

// Default implementations for major components
impl Default for Frame {
    fn default() -> Self {
        Self {
            geometry: FrameGeometry::default(),
            material: Material::Aluminum6061,
            construction_method: ConstructionMethod::Welded,
            tube_set: TubeSet::default(),
            weight: 1.8,
            stiffness_rating: 85.0,
            fatigue_life: 2000000,
        }
    }
}

impl Default for FrameGeometry {
    fn default() -> Self {
        // Road bike geometry
        Self {
            seat_tube_length: 540.0,
            top_tube_length: 560.0,
            head_tube_length: 140.0,
            chainstay_length: 405.0,
            wheelbase: 980.0,
            head_tube_angle: 73.0,
            seat_tube_angle: 74.0,
            bottom_bracket_height: 270.0,
            stack: 563.0,
            reach: 389.0,
            trail: 58.0,
            mechanical_trail: 65.0,
            front_center: 575.0,
        }
    }
}

impl Default for TubeSet {
    fn default() -> Self {
        let standard_tube = TubeSpec {
            diameter: 28.6,
            wall_thickness: 0.9,
            shape: TubeShape::Round,
            material: Material::Aluminum6061,
        };

        Self {
            seat_tube: standard_tube.clone(),
            top_tube: standard_tube.clone(),
            down_tube: TubeSpec {
                diameter: 34.9,
                wall_thickness: 1.2,
                shape: TubeShape::Round,
                material: Material::Aluminum6061,
            },
            head_tube: TubeSpec {
                diameter: 44.0,
                wall_thickness: 1.5,
                shape: TubeShape::Round,
                material: Material::Aluminum6061,
            },
            chainstays: standard_tube.clone(),
            seatstays: standard_tube,
        }
    }
}

impl Default for WheelSet {
    fn default() -> Self {
        let standard_wheel = Wheel::default();
        Self {
            front_wheel: standard_wheel.clone(),
            rear_wheel: standard_wheel,
            tire_clearance: 28.0,
        }
    }
}

impl Default for Wheel {
    fn default() -> Self {
        Self {
            diameter: 700.0, // 700c
            rim: RimSpec::default(),
            hub: HubSpec::default(),
            spokes: SpokeSpec::default(),
            tire: TireSpec::default(),
            weight: 0.9,
            rotational_inertia: 0.1,
        }
    }
}

impl Default for RimSpec {
    fn default() -> Self {
        Self {
            material: Material::Aluminum6061,
            depth: 23.0,
            width: 19.0,
            spoke_count: 32,
            aerodynamic_profile: AeroProfile::Traditional,
        }
    }
}

impl Default for HubSpec {
    fn default() -> Self {
        Self {
            material: Material::Aluminum6061,
            bearing_type: BearingType::Cartridge,
            engagement_points: 24,
            weight: 0.25,
        }
    }
}

impl Default for SpokeSpec {
    fn default() -> Self {
        Self {
            material: Material::StainlessSteel,
            count: 32,
            gauge: 2.0,
            pattern: SpokePattern::Cross3,
            tension: 1000.0,
        }
    }
}

impl Default for TireSpec {
    fn default() -> Self {
        Self {
            width: 25.0,
            pressure: 100.0,
            compound: TireCompound::Medium,
            tread_pattern: TreadPattern::SemiSlick,
            puncture_resistance: 5,
            rolling_resistance: 0.005,
        }
    }
}

impl Default for Drivetrain {
    fn default() -> Self {
        Self {
            crankset: Crankset::default(),
            cassette: Cassette::default(),
            chain: Chain::default(),
            front_derailleur: None,
            rear_derailleur: Derailleur::default(),
            shifters: ShifterSet::default(),
            bottom_bracket: BottomBracket::default(),
        }
    }
}

impl Default for Crankset {
    fn default() -> Self {
        Self {
            chainrings: vec![50, 34], // Compact double
            crank_length: 172.5,
            material: Material::Aluminum6061,
            weight: 0.8,
            power_meter: None,
        }
    }
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            speeds: 11,
            gear_ratios: vec![11, 12, 13, 14, 15, 17, 19, 21, 24, 28, 32],
            material: Material::Steel,
            weight: 0.3,
        }
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            speeds: 11,
            material: Material::Steel,
            length: 114,
            efficiency: 0.98,
        }
    }
}

impl Default for Derailleur {
    fn default() -> Self {
        Self {
            speeds: 11,
            capacity: 32,
            weight: 0.25,
        }
    }
}

impl Default for ShifterSet {
    fn default() -> Self {
        Self {
            speeds: 11,
            shift_type: ShiftType::Mechanical,
        }
    }
}

impl Default for BottomBracket {
    fn default() -> Self {
        Self {
            threading: ThreadingType::BSA,
            material: Material::Aluminum6061,
            bearing_type: BearingType::Cartridge,
        }
    }
}

impl Default for BrakingSystem {
    fn default() -> Self {
        Self {
            front_brake: Brake::default(),
            rear_brake: Brake::default(),
            brake_type: BrakeType::DiscBrake,
            stopping_power: 1500.0,
            modulation: 0.8,
        }
    }
}

impl Default for Brake {
    fn default() -> Self {
        Self {
            caliper: CaliperSpec::default(),
            rotor: Some(RotorSpec::default()),
            pads: PadSpec::default(),
        }
    }
}

impl Default for CaliperSpec {
    fn default() -> Self {
        Self {
            piston_count: 2,
            material: Material::Aluminum6061,
            weight: 0.15,
        }
    }
}

impl Default for RotorSpec {
    fn default() -> Self {
        Self {
            diameter: 160.0,
            material: Material::StainlessSteel,
            thickness: 1.8,
            mounting: RotorMounting::SixBolt,
        }
    }
}

impl Default for PadSpec {
    fn default() -> Self {
        Self {
            material: PadMaterial::Organic,
            life_expectancy: 3000,
        }
    }
}

impl Default for SteeringSystem {
    fn default() -> Self {
        Self {
            headset: HeadsetSpec::default(),
            stem: StemSpec::default(),
            handlebars: HandlebarSpec::default(),
        }
    }
}

impl Default for HeadsetSpec {
    fn default() -> Self {
        Self {
            bearing_type: BearingType::Cartridge,
            material: Material::Aluminum6061,
            stack_height: 40.0,
        }
    }
}

impl Default for StemSpec {
    fn default() -> Self {
        Self {
            length: 100.0,
            angle: -6.0,
            clamp_diameter: 31.8,
            material: Material::Aluminum6061,
        }
    }
}

impl Default for HandlebarSpec {
    fn default() -> Self {
        Self {
            style: HandlebarStyle::Drop,
            width: 420.0,
            drop: 125.0,
            reach: 80.0,
            material: Material::Aluminum6061,
        }
    }
}

impl Default for RiderFit {
    fn default() -> Self {
        Self {
            rider_height: 175.0,
            inseam: 84.0,
            arm_length: 60.0,
            torso_length: 58.0,
            flexibility: FlexibilityLevel::Medium,
            riding_style: RidingStyle::Balanced,
        }
    }
}
