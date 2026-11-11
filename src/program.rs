//! Mathematical program abstraction.

use crate::{Solution, SolverConfig};

/// Convex mathematical program with `N` primal variables, `E` unconstrained dual variables,
/// and `I` nonnegative dual variables.
/// 
/// Each unconstrained dual variable corresponds to an _equality_ constraint, and each
/// nonnegative dual variable corresponds to an _inequality_ constraint.
pub trait MathematicalProgram<const N: usize, const E: usize, const I: usize> {
    /// Solve this mathematical program by dispatching to the appropriate solver.
    fn solve(&self, config: SolverConfig) -> Solution<N, E, I>;
}
