//! Mathematical program abstraction.

use crate::{Solution, SolverConfig};

/// Convex mathematical program with `N` primal variables and `K` dual variables.
pub trait MathematicalProgram<const N: usize, const K: usize> {
    /// Solve this mathematical program by dispatching to the appropriate solver.
    fn solve(&self, config: SolverConfig) -> Solution<N, K>;
}
