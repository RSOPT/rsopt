//! Mathematical program abstraction.

use crate::{Solution, SolverConfig};

/// Convex mathematical program with `N` primal variables, `E` unconstrained dual variables,
/// and `I` nonnegative dual variables.
pub trait MathematicalProgram<const N: usize, const E: usize, const I: usize> {
    /// Solve this mathematical program by dispatching to the appropriate solver.
    fn solve(&self, config: SolverConfig) -> Solution<N, E, I>;
}
