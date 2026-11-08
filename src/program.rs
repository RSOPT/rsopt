//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Mathematical program abstraction.

use crate::Solution;

/// Convex mathematical program.
pub trait MathematicalProgram<const N: usize, const K: usize> {
    /// Solve this mathematical program by dispatching to the appropriate solver.
    fn solve(&self) -> Solution<N, K>;
}
