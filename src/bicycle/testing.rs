//! # Bicycle Testing Module
//!
//! This module provides comprehensive testing capabilities for bicycle designs,
//! including safety tests, performance benchmarks, and certification standards.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use super::design::BicycleDesign;
use super::physics::PhysicsEngine;

/// Test suite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<Test>,
    pub certification_standards: Vec<CertificationStandard>,
    pub pass_threshold: f64, // 0-100 percentage
}

/// Individual test definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub name: String,
    pub test_type: TestType,
    pub conditions: TestConditions,
    pub expected_results: ExpectedResults,
    pub weight: f64, // Test importance weight
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    SafetyTest,
    PerformanceTest,
    DurabilityTest,
    ComfortTest,
    EnvironmentalTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConditions {
    pub load: f64,           // kg
    pub cycles: u32,         // number of test cycles
    pub temperature: f64,    // celsius
    pub humidity: f64,       // percentage
    pub environment: TestEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestEnvironment {
    Laboratory,
    RoadTest,
    OffRoad,
    WeatherChamber,
    VibrationTable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedResults {
    pub min_value: f64,
    pub max_value: f64,
    pub target_value: f64,
    pub units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CertificationStandard {
    ISO4210,    // Safety requirements for bicycles
    EN14764,    // City and trekking bicycles
    EN14766,    // Mountain bicycles
    ASTMF1447,  // Static load test for bicycle forks
    CPSC,       // Consumer Product Safety Commission
    CE,         // European Conformity
    JIS,        // Japanese Industrial Standards
}

/// Test execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub design_id: Uuid,
    pub passed: bool,
    pub actual_value: f64,
    pub expected_range: (f64, f64),
    pub score: f64,
    pub notes: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Complete test session results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSessionResult {
    pub session_id: Uuid,
    pub design_id: Uuid,
    pub suite_name: String,
    pub individual_results: Vec<TestResult>,
    pub overall_score: f64,
    pub passed: bool,
    pub certification_status: HashMap<CertificationStandard, bool>,
    pub recommendations: Vec<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl TestSuite {
    /// Create a standard road bike test suite
    pub fn road_bike_standard() -> Self {
        Self {
            name: "Standard Road Bike Test Suite".to_string(),
            tests: vec![
                Test {
                    name: "Frame Fatigue Test".to_string(),
                    test_type: TestType::SafetyTest,
                    conditions: TestConditions {
                        load: 100.0,
                        cycles: 100000,
                        temperature: 20.0,
                        humidity: 50.0,
                        environment: TestEnvironment::Laboratory,
                    },
                    expected_results: ExpectedResults {
                        min_value: 100000.0,
                        max_value: f64::INFINITY,
                        target_value: 500000.0,
                        units: "cycles".to_string(),
                    },
                    weight: 0.3,
                },
                Test {
                    name: "Fork Static Load Test".to_string(),
                    test_type: TestType::SafetyTest,
                    conditions: TestConditions {
                        load: 1200.0,
                        cycles: 1,
                        temperature: 20.0,
                        humidity: 50.0,
                        environment: TestEnvironment::Laboratory,
                    },
                    expected_results: ExpectedResults {
                        min_value: 1200.0,
                        max_value: f64::INFINITY,
                        target_value: 2000.0,
                        units: "N".to_string(),
                    },
                    weight: 0.25,
                },
                Test {
                    name: "Braking Performance Test".to_string(),
                    test_type: TestType::PerformanceTest,
                    conditions: TestConditions {
                        load: 75.0,
                        cycles: 10,
                        temperature: 20.0,
                        humidity: 50.0,
                        environment: TestEnvironment::RoadTest,
                    },
                    expected_results: ExpectedResults {
                        min_value: 0.0,
                        max_value: 25.0,
                        target_value: 15.0,
                        units: "m".to_string(),
                    },
                    weight: 0.2,
                },
                Test {
                    name: "Comfort Rating Test".to_string(),
                    test_type: TestType::ComfortTest,
                    conditions: TestConditions {
                        load: 75.0,
                        cycles: 1,
                        temperature: 20.0,
                        humidity: 50.0,
                        environment: TestEnvironment::RoadTest,
                    },
                    expected_results: ExpectedResults {
                        min_value: 5.0,
                        max_value: 10.0,
                        target_value: 8.0,
                        units: "rating".to_string(),
                    },
                    weight: 0.15,
                },
                Test {
                    name: "Weight Efficiency Test".to_string(),
                    test_type: TestType::PerformanceTest,
                    conditions: TestConditions {
                        load: 0.0,
                        cycles: 1,
                        temperature: 20.0,
                        humidity: 50.0,
                        environment: TestEnvironment::Laboratory,
                    },
                    expected_results: ExpectedResults {
                        min_value: 0.0,
                        max_value: 12.0,
                        target_value: 8.0,
                        units: "kg".to_string(),
                    },
                    weight: 0.1,
                },
            ],
            certification_standards: vec![
                CertificationStandard::ISO4210,
                CertificationStandard::EN14764,
                CertificationStandard::CPSC,
            ],
            pass_threshold: 75.0,
        }
    }

    /// Create a mountain bike test suite
    pub fn mountain_bike_standard() -> Self {
        let mut suite = Self::road_bike_standard();
        suite.name = "Mountain Bike Test Suite".to_string();
        suite.certification_standards = vec![
            CertificationStandard::ISO4210,
            CertificationStandard::EN14766,
            CertificationStandard::CPSC,
        ];
        
        // Add mountain bike specific tests
        suite.tests.push(Test {
            name: "Suspension Performance Test".to_string(),
            test_type: TestType::PerformanceTest,
            conditions: TestConditions {
                load: 100.0,
                cycles: 10000,
                temperature: 20.0,
                humidity: 50.0,
                environment: TestEnvironment::VibrationTable,
            },
            expected_results: ExpectedResults {
                min_value: 80.0,
                max_value: 120.0,
                target_value: 100.0,
                units: "mm".to_string(),
            },
            weight: 0.2,
        });

        suite.tests.push(Test {
            name: "Off-Road Durability Test".to_string(),
            test_type: TestType::DurabilityTest,
            conditions: TestConditions {
                load: 90.0,
                cycles: 50000,
                temperature: 25.0,
                humidity: 70.0,
                environment: TestEnvironment::OffRoad,
            },
            expected_results: ExpectedResults {
                min_value: 50000.0,
                max_value: f64::INFINITY,
                target_value: 200000.0,
                units: "cycles".to_string(),
            },
            weight: 0.25,
        });

        suite
    }

    /// Execute the test suite on a bicycle design
    pub fn execute(&self, design: &BicycleDesign, physics_engine: &PhysicsEngine) -> TestSessionResult {
        let session_id = Uuid::new_v4();
        let started_at = chrono::Utc::now();
        let mut individual_results = Vec::new();
        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;

        for test in &self.tests {
            let result = self.execute_single_test(test, design, physics_engine);
            total_weighted_score += result.score * test.weight;
            total_weight += test.weight;
            individual_results.push(result);
        }

        let overall_score = if total_weight > 0.0 {
            total_weighted_score / total_weight
        } else {
            0.0
        };

        let passed = overall_score >= self.pass_threshold;
        
        let certification_status = self.evaluate_certifications(&individual_results);
        let recommendations = self.generate_recommendations(&individual_results);

        TestSessionResult {
            session_id,
            design_id: design.id,
            suite_name: self.name.clone(),
            individual_results,
            overall_score,
            passed,
            certification_status,
            recommendations,
            started_at,
            completed_at: chrono::Utc::now(),
        }
    }

    fn execute_single_test(&self, test: &Test, design: &BicycleDesign, physics_engine: &PhysicsEngine) -> TestResult {
        let actual_value = match test.test_type {
            TestType::SafetyTest => {
                match test.name.as_str() {
                    "Frame Fatigue Test" => design.frame.fatigue_life as f64,
                    "Fork Static Load Test" => design.frame.stiffness_rating * 20.0,
                    _ => 100.0,
                }
            },
            TestType::PerformanceTest => {
                match test.name.as_str() {
                    "Braking Performance Test" => {
                        // Calculate braking distance
                        super::physics::calculate_braking_distance(15.0, design.braking.stopping_power, 80.0, 0.7)
                    },
                    "Weight Efficiency Test" => design.calculate_total_weight(),
                    "Suspension Performance Test" => {
                        design.suspension.as_ref()
                            .map(|s| s.total_travel)
                            .unwrap_or(0.0)
                    },
                    _ => 50.0,
                }
            },
            TestType::ComfortTest => {
                // Simple comfort calculation based on frame stiffness
                10.0 - (design.frame.stiffness_rating / 15.0)
            },
            TestType::DurabilityTest => {
                match test.name.as_str() {
                    "Off-Road Durability Test" => design.frame.fatigue_life as f64 * 0.7,
                    _ => design.frame.fatigue_life as f64,
                }
            },
            TestType::EnvironmentalTest => 75.0,
        };

        let passed = actual_value >= test.expected_results.min_value && 
                     actual_value <= test.expected_results.max_value;
        
        let score = if passed {
            let range = test.expected_results.max_value - test.expected_results.min_value;
            if range > 0.0 {
                ((actual_value - test.expected_results.min_value) / range * 100.0).min(100.0)
            } else {
                100.0
            }
        } else {
            0.0
        };

        let notes = if !passed {
            vec![format!("Test failed: actual value {} outside expected range {}-{}", 
                        actual_value, test.expected_results.min_value, test.expected_results.max_value)]
        } else {
            vec!["Test passed successfully".to_string()]
        };

        TestResult {
            test_name: test.name.clone(),
            design_id: design.id,
            passed,
            actual_value,
            expected_range: (test.expected_results.min_value, test.expected_results.max_value),
            score,
            notes,
            timestamp: chrono::Utc::now(),
        }
    }

    fn evaluate_certifications(&self, results: &[TestResult]) -> HashMap<CertificationStandard, bool> {
        let mut certification_status = HashMap::new();
        
        for standard in &self.certification_standards {
            let required_tests = self.get_required_tests_for_certification(standard);
            let all_passed = required_tests.iter()
                .all(|test_name| {
                    results.iter()
                        .find(|r| &r.test_name == test_name)
                        .map(|r| r.passed)
                        .unwrap_or(false)
                });
            
            certification_status.insert(standard.clone(), all_passed);
        }
        
        certification_status
    }

    fn get_required_tests_for_certification(&self, standard: &CertificationStandard) -> Vec<String> {
        match standard {
            CertificationStandard::ISO4210 => vec![
                "Frame Fatigue Test".to_string(),
                "Fork Static Load Test".to_string(),
                "Braking Performance Test".to_string(),
            ],
            CertificationStandard::EN14764 => vec![
                "Frame Fatigue Test".to_string(),
                "Comfort Rating Test".to_string(),
            ],
            CertificationStandard::EN14766 => vec![
                "Frame Fatigue Test".to_string(),
                "Suspension Performance Test".to_string(),
                "Off-Road Durability Test".to_string(),
            ],
            CertificationStandard::CPSC => vec![
                "Fork Static Load Test".to_string(),
                "Braking Performance Test".to_string(),
            ],
            _ => vec![],
        }
    }

    fn generate_recommendations(&self, results: &[TestResult]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for result in results {
            if !result.passed {
                match result.test_name.as_str() {
                    "Frame Fatigue Test" => {
                        recommendations.push("Consider upgrading frame material or increasing tube wall thickness".to_string());
                    },
                    "Fork Static Load Test" => {
                        recommendations.push("Strengthen fork design or use higher-grade materials".to_string());
                    },
                    "Braking Performance Test" => {
                        recommendations.push("Improve braking system with larger rotors or better brake pads".to_string());
                    },
                    "Weight Efficiency Test" => {
                        recommendations.push("Reduce weight through material optimization or component selection".to_string());
                    },
                    "Comfort Rating Test" => {
                        recommendations.push("Improve ride comfort with better geometry or vibration damping".to_string());
                    },
                    _ => {
                        recommendations.push(format!("Address issues with {}", result.test_name));
                    }
                }
            }
        }
        
        if recommendations.is_empty() {
            recommendations.push("All tests passed! Design meets certification requirements.".to_string());
        }
        
        recommendations
    }
}
