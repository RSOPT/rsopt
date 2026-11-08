//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Unconstrained quadratic program.

use nalgebra::{SMatrix, SVector};

use crate::{Float, MathematicalProgram, Solution};

/// Unconstrained quadratic program.
/// 
/// This implementation of unconstrained QP uses a "direct solution" method.
pub struct UnconstrainedQuadraticProgram<const N: usize> {
    #[allow(dead_code)]
    /// Matrix `A`.
    matrix: SMatrix<Float, N, N>,

    #[allow(dead_code)]
    /// Vector `b`.
    vector: SVector<Float, N>,
}

impl<const N: usize> MathematicalProgram<N, 0> for UnconstrainedQuadraticProgram<N> {
    fn solve(&self) -> Solution<N, 0> {
        todo!()
    }
}
