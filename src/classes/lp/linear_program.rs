//! Linear program.

#[allow(unused_imports)]
use nalgebra::{SMatrix, SVector};

#[allow(unused_imports)]
use crate::{Float, MathematicalProgram, Solution, SolverConfig, Status};

/// Linear program.
pub struct LinearProgram<const N: usize, const E: usize, const I: usize> {

}

impl<const N: usize, const E: usize, const I: usize> LinearProgram<N, E, I> {
    /// Construct this linear program.
    pub fn new() -> Self {
        Self {

        }
    }
}

impl<const N: usize, const E: usize, const I: usize> MathematicalProgram<N, E, I> for LinearProgram<N, E, I> {
    fn solve(&self, _config: SolverConfig) -> Solution<N, E, I> {
        todo!()
    }
}
