//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Solver configuration abstraction.

use core::default::Default;

use crate::Float;

#[derive(Clone, Copy)]
/// Solver configuration abstraction.
pub struct SolverConfig {
    /// Gradient tolerance of the primal objective.
    pub primal_gradient_tolerance: Float,    
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            primal_gradient_tolerance: 1e-8,
        }
    }
}
