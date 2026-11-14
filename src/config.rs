//! Solver configuration abstraction.

use core::default::Default;

use crate::Float;

#[derive(Clone, Copy)]
/// Solver configuration abstraction.
pub struct SolverConfig {
    /// Gradient tolerance of the primal objective.
    pub primal_gradient_tolerance: Float,    

    /// Major iteration limit.
    pub major_iteration_limit: usize,

    /// Minor iteration limit.
    pub minor_iteration_limit: usize,

    /// Regularization coefficient.
    pub regularization: Float,

    /// Fraction-to-the-boundary coefficient.
    pub boundary_coefficient: Float,

    /// Line search scaling fraction.
    pub line_search_fraction: Float,

    /// Primal update L1 norm threshold.
    pub primal_update_norm_threshold: Float,

    /// Initial barrier parameter.
    pub initial_barrier_parameter: Float,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            primal_gradient_tolerance: 1e-8,
            major_iteration_limit: 20,
            minor_iteration_limit: 15,
            regularization: 1e-9,
            boundary_coefficient: 0.995,
            line_search_fraction: 0.5,
            primal_update_norm_threshold: 1e-8,
            initial_barrier_parameter: 10.0,
        }
    }
}
