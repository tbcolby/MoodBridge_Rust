//! # Computational Engine Plugins
//! 
//! This module contains implementations of various computational engine plugins.

pub mod wolfram_alpha;

// Re-export main engine implementations
pub use wolfram_alpha::{WolframAlphaEngine, WolframAlphaConfig};

// Additional engines can be added here:
// pub mod sympy;
// pub mod matlab;
// pub mod mathematica;
// pub mod openai_codex;
